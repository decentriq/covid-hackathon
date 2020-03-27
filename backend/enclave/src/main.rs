extern crate rs_libc;
extern crate rs_libc_decentriq;
extern crate spatialite_sys;
use hyper::server::Request;
use hyper::server::Response;
use rusqlite::params;
use rusqlite::Connection;
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
use log::error;

type UserId = String;
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

struct Configuration {
    // The time distance under which a point is considered exposed wrt an infected point.
    exposure_time_distance: Duration,
    // The space distance under which a point is considered exposed wrt an infected point.
    exposure_space_distance: f32,
    // The interval of the enclave scanning and updating its state.
    update_interval: Duration,
}

struct EnclaveState {
    connection: Connection,
    exposed: HashMap<UserId, DateTime<Utc>>,
    last_update: Option<DateTime<Utc>>,

    temp_in_memory: Vec<TempEntry>,
}

struct TempEntry {
    user_id: UserId,
    infected: bool,
    timestamped_coordinate: TimestampedCoordinate,
}

struct EnclaveHandler {
    state: Arc<Mutex<EnclaveState>>,
    configuration: Configuration,
}

#[derive(Debug)]
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
    fn new(connection: Connection) -> EnclaveHandler {
        let state = EnclaveState {
            connection,
            exposed: HashMap::new(),
            last_update: None,
            temp_in_memory: vec![],
        };
        let utc: DateTime<Utc> = Utc::now();
        let asd: Duration = utc.sub(utc);
        EnclaveHandler {
            state: Arc::new(Mutex::new(state)),
            configuration: Configuration::default(),
        }
    }

    fn handle_poll(&self, req: &mut Request) -> Result<Vec<u8>> {
        let mut poll_request: PollRequest = serde_json::from_reader(req)?;
        let poll_response = {
            let mut guard = self.state.lock().unwrap();
            EnclaveHandler::insert_new_points(guard.borrow_mut(), &mut poll_request);
            EnclaveHandler::update_if_necessary(guard.borrow_mut(), &poll_request, &self.configuration);
            EnclaveHandler::query_exposed(guard.borrow(), &poll_request)
        };
        info!("Poll response {:?}", poll_response);
        Ok(serde_json::to_vec(&poll_response).unwrap())
    }

    fn update_if_necessary(state: &mut EnclaveState, request: &PollRequest, configuration: &Configuration) {
        let latest_timestamped_coordinate = request.timestamped_coordinates.iter().max_by(|a, b| {
            a.timestamp.cmp(&b.timestamp)
        });
        let latest_timestamp = match latest_timestamped_coordinate {
            None => {
                // Don't update if there are no timestamps in the request at all
                return
            },
            Some(coord) => {
                // If there was at least one update already..
                if let Some(last_update) = state.last_update {
                    // .. but not enough time has passed ..
                    if coord.timestamp - last_update < configuration.update_interval {
                        // .. then don't update ..
                        return
                    }
                }
                // Otherwise update
                coord.timestamp
            }
        };

        info!("Updating database");

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

        state.last_update = Some(latest_timestamp);
    }

    fn insert_new_points(state: &mut EnclaveState, request: &mut PollRequest) {
        info!("Inserting new points of {}", request.user_id);

        for timestamped_coordinate in request.timestamped_coordinates.drain(0..) {
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

impl Default for Configuration {
    fn default() -> Self {
        Configuration {
            exposure_time_distance: Duration::seconds(30),
            exposure_space_distance: 3.0,
            update_interval: Duration::seconds(1),
        }
    }
}

fn main() {
    setup_logging();
    let connection = Connection::open_in_memory().unwrap();
    let enclave_handler = EnclaveHandler::new(connection);
    let _listening = hyper::Server::http("[::]:3000").unwrap().handle(enclave_handler);
    println!("Listening on http://[::]:3000");
}

fn setup_logging() {
    std::env::set_var("RUST_LOG", "info");
    env_logger::Builder::from_default_env()
        .target(env_logger::Target::Stdout)
        .init();
}
