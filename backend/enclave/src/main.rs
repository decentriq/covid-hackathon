use hyper::server::Request;
use hyper::server::Response;
use std::io::Read;
use std::collections::{HashSet, HashMap};
use hyper::uri::RequestUri::AbsolutePath;
use hyper::net::Fresh;
use std::sync::Arc;
use std::sync::Mutex;
use serde::Deserialize;
use serde::Serialize;
use chrono::DateTime;
use chrono::Utc;
use time::Duration;
use std::ops::Sub;
use std::borrow::{BorrowMut, Borrow};
use log::info;
use log::warn;
use log::error;
use sgx_isa::Report;
use sgx_isa::Targetinfo;

type UserId = String;
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
type Keypair = ();

struct Configuration {
    // The time distance under which a point is considered exposed wrt an infected point.
    exposure_time_distance: Duration,
    // The space distance under which a point is considered exposed wrt an infected point.
    exposure_space_distance: f32,
    // The interval of the enclave scanning and updating its state. If 0 then every request will
    // trigger an update.
    update_interval: Duration,
}

impl Default for Configuration {
    fn default() -> Self {
        Configuration {
            exposure_time_distance: Duration::seconds(30),
            exposure_space_distance: 3.0,
            update_interval: Duration::seconds(0),
        }
    }
}

#[derive(Debug)]
struct EnclaveState {
    exposed: HashMap<UserId, DateTime<Utc>>,
    last_update: Option<DateTime<Utc>>,

    temp_in_memory: Vec<TempEntry>,
}

#[derive(Debug)]
struct TempEntry {
    user_id: UserId,
    infected: bool,
    timestamped_coordinate: TimestampedCoordinate,
}

struct EnclaveHandler {
    state: Arc<Mutex<EnclaveState>>,
    keypair: Keypair,
    report: Report,
    configuration: Configuration,
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(Serialize, Deserialize)]
struct TimestampedCoordinate {
    timestamp: DateTime<Utc>,
    x: f32,
    y: f32,
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
struct PollRequest {
    user_id: UserId,
    infected: bool,
    timestamped_coordinates: Vec<TimestampedCoordinate>,
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
struct PollResponse {
    exposed_timestamp: Option<DateTime<Utc>>,
}

impl EnclaveHandler {
    fn new(keypair: Keypair, report: Report) -> EnclaveHandler {
        let state = EnclaveState {
            exposed: HashMap::new(),
            last_update: None,
            temp_in_memory: vec![],
        };
        let utc: DateTime<Utc> = Utc::now();
        let asd: Duration = utc.sub(utc);
        EnclaveHandler {
            state: Arc::new(Mutex::new(state)),
            keypair,
            report,
            configuration: Configuration::default(),
        }
    }

    fn decrypt<R: Read>(_keypair: Keypair, reader: R) -> R {
        warn!("TODO decrypt");
        reader
    }

    fn encrypt(_keypair: Keypair, input: Vec<u8>) -> Vec<u8> {
        warn!("TODO encrypt");
        input
    }

    fn handle_poll(&self, req: &mut Request) -> Result<Vec<u8>> {
        let decrypted = EnclaveHandler::decrypt(self.keypair, req);
        let mut poll_request: PollRequest = serde_json::from_reader(decrypted)?;
        let poll_response = {
            let mut guard = self.state.lock().unwrap();
            EnclaveHandler::insert_new_points(guard.borrow_mut(), &poll_request);
            EnclaveHandler::update_if_necessary(guard.borrow_mut(), &poll_request, &self.configuration);
            EnclaveHandler::query_exposed(guard.borrow(), &poll_request)
        };
        info!("Poll response {:?}", poll_response);
        let serialized_response = serde_json::to_vec(&poll_response).unwrap();
        let encrypted = EnclaveHandler::encrypt(self.keypair, serialized_response);
        Ok(encrypted)
    }

    fn get_latest_timestamp(request: &PollRequest) -> Option<DateTime<Utc>> {
        request.timestamped_coordinates.iter().max_by(|a, b| {
            a.timestamp.cmp(&b.timestamp)
        }).map(|timestamped_coordinate|timestamped_coordinate.timestamp)
    }

    fn update_if_necessary(state: &mut EnclaveState, poll_request: &PollRequest, configuration: &Configuration) {
        let latest_timestamp = EnclaveHandler::get_latest_timestamp(poll_request);
        info!("Latest timestamp in request {:?}", latest_timestamp);
        let latest_timestamp_unwrapped = match latest_timestamp {
            None => {
                // Don't update if there are no timestamps in the request at all
                return
            },
            Some(timestamp) => {
                // If there was at least one update already..
                if let Some(last_update) = state.last_update {
                    // .. but not enough time has passed ..
                    if timestamp - last_update < configuration.update_interval {
                        // .. then don't update ..
                        return
                    }
                }
                // Otherwise update
                timestamp
            }
        };

        info!("Updating database, enclave state {:?}", state);

        let delta_squared = configuration.exposure_space_distance * configuration.exposure_space_distance;
        for entry in &state.temp_in_memory {
            if let Some(exposure_time) = state.exposed.get(&entry.user_id) {
                // If we know the user is marked exposed with a more recent timestamp, don't scan.
                if exposure_time > &entry.timestamped_coordinate.timestamp {
                    continue
                }
            }
            let coord = &entry.timestamped_coordinate;
            for other_entry in &state.temp_in_memory {
                if other_entry.user_id == entry.user_id {
                    continue
                }
                let dx = entry.timestamped_coordinate.x - other_entry.timestamped_coordinate.x;
                let dy = entry.timestamped_coordinate.y - other_entry.timestamped_coordinate.y;
                let dtime = entry.timestamped_coordinate.timestamp - other_entry.timestamped_coordinate.timestamp;
                if dtime < configuration.exposure_time_distance && dx * dx + dy * dy < delta_squared {
                    state.exposed.insert(entry.user_id.clone(), entry.timestamped_coordinate.timestamp);
                    break
                }
            }
        }

        state.last_update = Some(latest_timestamp_unwrapped);
    }

    fn insert_new_points(state: &mut EnclaveState, request: &PollRequest) {
        info!("Inserting new points of {}", request.user_id);

        for timestamped_coordinate in request.timestamped_coordinates.clone() {
            state.temp_in_memory.push(TempEntry {
                user_id: request.user_id.clone(),
                infected: request.infected,
                timestamped_coordinate
            });
        }
    }

    fn query_exposed(state: &EnclaveState, request: &PollRequest) -> PollResponse {
        info!("Querying exposed status of {}", request.user_id);
        PollResponse {
            exposed_timestamp: state.exposed.get(&request.user_id).cloned()
        }
    }
}

impl hyper::server::Handler for EnclaveHandler {
    fn handle(&self, mut req: Request, mut res: Response) {
        match req.uri {
            AbsolutePath(ref path) => match (&req.method, &path[..]) {
                (&hyper::Post, "/poll") => {
                    match self.handle_poll(&mut req) {
                        Ok(response_contents) => {
                            res.send(response_contents.as_slice()).unwrap();
                        }
                        Err(err) => {
                            error!("Error during request processing: {:?}", err);
                            *res.status_mut() = hyper::BadRequest;
                            res.send(format!("{:?}", err).as_bytes()).unwrap();
                        }
                    }
                }
                _ => {
                    *res.status_mut() = hyper::NotFound;
                    res.start().unwrap();
                }
            },
            _ => {
                *res.status_mut() = hyper::NotFound;
                res.start().unwrap();
            }
        }
    }
}

fn main() {
    setup_logging();
    let keypair = generate_keypair();
    let report = create_report(keypair);
    let enclave_handler = EnclaveHandler::new(keypair, report);
    let _listening = hyper::Server::http("[::]:3000").unwrap().handle(enclave_handler);
    println!("Listening on http://[::]:3000");
}

fn create_report(keypair: Keypair) -> Report {
    let target_info = Targetinfo::default();
    warn!("TODO create_report fill report_data");
    let report_data = [0; 64];
    Report::for_target(&target_info, &report_data)
}

fn generate_keypair() -> Keypair {
    warn!("TODO generate_keypair");
}

fn setup_logging() {
    std::env::set_var("RUST_LOG", "info");
    env_logger::Builder::from_default_env()
        .target(env_logger::Target::Stdout)
        .init();
}
