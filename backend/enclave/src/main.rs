extern crate rs_libc;
extern crate rs_libc_decentriq;
use hyper::server::Request;
use hyper::server::Response;
use rusqlite::params;
use rusqlite::Connection;
use rusqlite::Result;
use std::io::Read;
use std::collections::HashSet;
use hyper::uri::RequestUri::AbsolutePath;
use hyper::net::Fresh;
use std::sync::{Arc, Mutex};

type UserId = String;

struct EnclaveState {
    connection: Connection,
    to_notify: HashSet<UserId>,
}

struct EnclaveHandler {
    state: Arc<Mutex<EnclaveState>>,
}

impl EnclaveHandler {
    fn new(connection: Connection) -> EnclaveHandler {
        let state = EnclaveState {
            connection,
            to_notify: HashSet::new(),
        };

        EnclaveHandler { state: Arc::new(Mutex::new(state)) }
    }


    fn handle_poll(&self, req: &mut Request, res: &mut Response) {

    }
}

impl hyper::server::Handler for EnclaveHandler {
    fn handle(&self, mut req: Request, mut res: Response) {
        match req.uri {
            AbsolutePath(ref path) => match (&req.method, &path[..]) {
                (&hyper::Post, "/poll") => {
                    handle_poll(&mut req, &mut res);
                }
                _ => {
                    *res.status_mut() = hyper::NotFound;
                }
            },
            _ => {
                *res.status_mut() = hyper::NotFound;
            }
        }

        let mut res = try_return!(res.start());
        try_return!(copy(&mut req, &mut res));
    }
}

fn process_main(mut req: Request, mut res: Response) {
}

fn main() {
    let _listening = hyper::Server::http("127.0.0.1:3000").unwrap().handle(process_main);
    println!("Listening on http://127.0.0.1:3000");
}
