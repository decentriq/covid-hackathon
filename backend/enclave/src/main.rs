use hyper::server::Request;
use hyper::server::Response;
use std::io::Read;
use std::collections::{HashMap};
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
use nav_types::{ WGS84, ECEF };
use kdtree::KdTree;
use chily::*;

type UserId = String;
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

struct Configuration {
    // The time distance under which a point is considered exposed wrt an infected point.
    exposure_time_distance: Duration,
    // The space distance under which a point is considered exposed wrt an infected point.
    exposure_space_distance: f32,
    // The interval of the enclave scanning and updating its state. If 0 then every request will
    // trigger an update.
    update_interval: Duration,
    // The time before a user is set as infected in which the user is considered to be exposed.
    infectious_period: Duration,
    // The number of points to store in a KD tree node before splitting.
    kdtree_capacity: usize
}

impl Default for Configuration {
    fn default() -> Self {
        Configuration {
            exposure_time_distance: Duration::seconds(30),
            exposure_space_distance: 3.0,
            update_interval: Duration::seconds(1),
            infectious_period: Duration::weeks(2),
            kdtree_capacity: 50
        }
    }
}

#[derive(Debug)]
struct User {
    gps_track: Vec<TimestampedCoordinate>,
    illnesses: Vec<Illness>,
    exposure_time: Option<DateTime<Utc>>
}

impl User {
    pub fn was_ill_at(&self, time: &DateTime<Utc>) -> bool {
        for illness in &self.illnesses {
            if illness.start_time < *time {
                match illness.duration_days {
                    None => return true,
                    Some(duration_days) =>
                        if *time - illness.start_time < Duration::days(duration_days) {
                            return true;
                        }
                }
            }
        }
        return false;
    }
}

#[derive(Debug)]
struct EnclaveState {
    users: HashMap<UserId, User>,
    last_update: Option<DateTime<Utc>>,
}

struct EnclaveHandler {
    state: Arc<Mutex<EnclaveState>>,
    keypair: Keypair,
    report: Report,
    configuration: Configuration,
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
#[derive(Clone)]
struct Illness {
    start_time: DateTime<Utc>,
    duration_days: Option<i64> // none = ongoing
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
#[derive(Clone)]
struct TimestampedCoordinate {
    timestamp: DateTime<Utc>,
    // WGS-84 coordinates
    x: f32,
    y: f32,
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
struct PollRequest {
    user_id: UserId,
    illnesses: Vec<Illness>,
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
            last_update: None,
            users: HashMap::new(),
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

    fn decrypt(&self, client_key: &PublicKey, input: Vec<u8>) -> Vec<u8> {
        let cipher = Cipher::new(&self.keypair.secret, client_key);
        let nonce: Nonce = input.as_slice().into();
        let encrypted_content = &input[24..];
        cipher.decrypt(encrypted_content, &nonce)
    }

    fn encrypt(&self, client_key: &PublicKey, input: Vec<u8>) -> Vec<u8> {
        let cipher = Cipher::new(&self.keypair.secret, client_key);
        let nonce = Nonce::from_random();
        let encrypted_content = cipher.encrypt(&input, &nonce);
        let mut complete = nonce.bytes.to_vec();
        complete.extend(encrypted_content);
        complete
    }

    fn handle_poll(&self, req: &mut Request) -> Result<Vec<u8>> {
        let mut body_content = vec![];
        req.read_to_end(&mut body_content)?;
        // TODO: extract from header
        let mut client_pubkey_buf: [u8; 32] = [0; 32];
        let client_pubkey: PublicKey = client_pubkey_buf.into();
        let decrypted = self.decrypt(&client_pubkey, body_content);
        let mut poll_request: PollRequest = serde_json::from_slice(&decrypted)?;
        let poll_response = {
            let mut guard = self.state.lock().unwrap();
            EnclaveHandler::insert_new_points(guard.borrow_mut(), &poll_request);
            EnclaveHandler::update_if_necessary(guard.borrow_mut(), &poll_request, &self.configuration);
            EnclaveHandler::query_exposed(guard.borrow(), &poll_request)
        };
        info!("Poll response {:?}", poll_response);
        let serialized_response = serde_json::to_vec(&poll_response).unwrap();
        let encrypted = self.encrypt(&client_pubkey, serialized_response);
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

        // Build a KD tree containing all the tracks.
        // TODO: this kd tree could be updated dynamically, but then it would need to be rebalanced
        let mut kdtree = KdTree::with_capacity(4, configuration.kdtree_capacity);
        for ( user_id, user ) in &state.users {
            for entry in &user.gps_track {
                let pos = WGS84::new(entry.x as f64, entry.y as f64, 0.0);
                let ecef_pos = ECEF::from(pos);

                let point: [f64; 4] = [
                    ecef_pos.x(),
                    ecef_pos.y(),
                    ecef_pos.z(),
                    entry.timestamp.timestamp() as f64
                ];

                kdtree.add(point, user_id).unwrap();
            }
        }

        let space_limit = configuration.exposure_space_distance as f64;
        let time_limit = configuration.exposure_time_distance;
        let mut exposure_times: HashMap<UserId, DateTime<Utc>> = HashMap::new();
        for ( user_id, user ) in &state.users {
            if user.illnesses.is_empty() {
                continue;
            }

            for entry in &user.gps_track {
                if !user.was_ill_at(&entry.timestamp) {
                    continue;
                }

                // Find people who were exposed.
                let pos = WGS84::new(entry.x as f64, entry.y as f64, 0.0);
                let ecef_pos = ECEF::from(pos);

                let range_min: [f64; 4] = [
                    ecef_pos.x() - space_limit,
                    ecef_pos.y() - space_limit,
                    ecef_pos.z() - space_limit,
                    (entry.timestamp - time_limit).timestamp() as f64
                ];

                let range_max: [f64; 4] = [
                    ecef_pos.x() + space_limit,
                    ecef_pos.y() + space_limit,
                    ecef_pos.z() + space_limit,
                    (entry.timestamp + time_limit).timestamp() as f64
                ];

                for other_user_id in kdtree.in_range(&range_min, &range_max) {
                    match exposure_times.get_mut(*other_user_id) {
                        None => { exposure_times.insert(other_user_id.to_string(), entry.timestamp); },
                        Some(cur_exposure_time) => {
                            if entry.timestamp > *cur_exposure_time {
                                *cur_exposure_time = entry.timestamp;
                            }
                        }
                    }
                }
            }
        }

        for ( user_id, exposure_time ) in exposure_times {
            if let Some(user) = state.users.get_mut(&user_id) {
                user.exposure_time = Some(exposure_time);
            }
        }

        state.last_update = Some(latest_timestamp_unwrapped);
    }

    fn insert_new_points(state: &mut EnclaveState, request: &PollRequest) {
        info!("Inserting new points of {}", request.user_id);
        match state.users.get_mut(&request.user_id) {
            Some(user) => {
                user.illnesses = request.illnesses.clone();
                for timestamped_coordinate in &request.timestamped_coordinates {
                    user.gps_track.push(timestamped_coordinate.clone());
                }
            },
            None => {
                info!("Creating new user {}", request.user_id);
                state.users.insert(request.user_id.clone(), User {
                    illnesses: request.illnesses.clone(),
                    exposure_time: None,
                    gps_track: request.timestamped_coordinates.clone()
                });
            }
        }
    }

    fn query_exposed(state: &EnclaveState, request: &PollRequest) -> PollResponse {
        info!("Querying exposed status of {}", request.user_id);
        PollResponse {
            exposed_timestamp: state.users.get(&request.user_id).and_then(|user| user.exposure_time.clone())
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
                (&hyper::Get, "/report") => {
                    *res.status_mut() = hyper::Ok;
                    res.send(self.report.as_ref()).unwrap();
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
    let report = create_report(&keypair);
    let enclave_handler = EnclaveHandler::new(keypair, report);
    let _listening = hyper::Server::http("[::]:3000").unwrap().handle(enclave_handler);
    println!("Listening on http://[::]:3000");
}

fn create_report(keypair: &Keypair) -> Report {
    let target_info = Targetinfo::default();
    warn!("TODO create_report fill report_data");
    let mut report_data: [u8; 64] = [0; 64];
    let public_key_bytes = keypair.public.as_bytes();
    &report_data[..public_key_bytes.len()].copy_from_slice(public_key_bytes);
    Report::for_target(&target_info, &report_data)
}

fn generate_keypair() -> Keypair {
    Keypair::generate()
}

fn setup_logging() {
    std::env::set_var("RUST_LOG", "info");
    env_logger::Builder::from_default_env()
        .target(env_logger::Target::Stdout)
        .init();
}
