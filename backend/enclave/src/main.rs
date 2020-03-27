extern crate rs_libc;
extern crate rs_libc_decentriq;
use hyper::server::Request;
use hyper::server::Response;
use rusqlite::params;
use rusqlite::Connection;
use rusqlite::Result;
use std::io::Read;

fn hello(mut req: Request, res: Response) {
    let mut buffer = Vec::new();

    req.read_to_end(&mut buffer);
    let req_str = std::str::from_utf8(&buffer).unwrap();

    let conn = Connection::open_in_memory().unwrap();

    res.send(req_str.as_bytes()).unwrap();
}

fn main() {
    let _listening = hyper::Server::http("127.0.0.1:3000").unwrap().handle(hello);
    println!("Listening on http://127.0.0.1:3000");
}
