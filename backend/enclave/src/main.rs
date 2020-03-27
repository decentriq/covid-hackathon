extern crate rs_libc;
extern crate rs_libc_decentriq;

use rusqlite::params;
use rusqlite::Connection;
use rusqlite::Result;

fn main() {
    let conn = Connection::open_in_memory().unwrap();
}
