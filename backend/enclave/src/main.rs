extern crate rs_libc;
extern crate rs_libc_decentriq;
use hyper::server::Request;
use hyper::server::Response;
use rusqlite::params;
use rusqlite::Connection;
use std::io::Read;
use std::collections::HashSet;
use hyper::uri::RequestUri::AbsolutePath;
use hyper::net::Fresh;
use std::sync::Arc;
use std::sync::Mutex;
use serde::Deserialize;
use serde::Serialize;
use chrono::DateTime;
use chrono::Utc;

type UserId = String;
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

struct EnclaveState {
    connection: Connection,
    to_notify: HashSet<UserId>,
}

struct EnclaveHandler {
    state: Arc<Mutex<EnclaveState>>,
}

#[derive(Serialize, Deserialize)]
struct TimestampedCoordinate {
    timestamp: DateTime<Utc>,
    x: f32,
    y: f32,
}

#[derive(Serialize, Deserialize)]
struct PollRequest {
    user_id: UserId,
    timestamped_coordinates: Vec<TimestampedCoordinate>,
}

#[derive(Serialize, Deserialize)]
struct PollResponse {

}

impl EnclaveHandler {
    fn new(connection: Connection) -> EnclaveHandler {
        let state = EnclaveState {
            connection,
            to_notify: HashSet::new(),
        };

        EnclaveHandler { state: Arc::new(Mutex::new(state)) }
    }

    fn handle_poll(&self, req: &mut Request) -> Result<Vec<u8>> {
        let poll_request: PollRequest = serde_json::from_reader(req)?;
        Ok(serde_json::to_vec(&poll_request).unwrap())
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
    let connection = Connection::open_in_memory().unwrap();
    let enclave_handler = EnclaveHandler::new(connection);
    let _listening = hyper::Server::http("[::]:3000").unwrap().handle(enclave_handler);
    println!("Listening on http://[::]:3000");
}

fn setup_logging() {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::Builder::from_default_env()
        .target(env_logger::Target::Stdout)
        .init();
}
