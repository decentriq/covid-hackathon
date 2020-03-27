#![feature(static_nobundle)]

use std::os::raw::{c_int};
use rusqlite::Connection;

pub enum SpatialiteState {
}

#[link(name = "spatialite")]
extern {
  fn spatialite_alloc_connection() -> *mut SpatialiteState;
  fn spatialite_init_ex(db_handle: *mut libsqlite3_sys::sqlite3, ptr: *mut SpatialiteState, verbose: c_int);
  fn spatialite_cleanup_ex(ptr: *mut SpatialiteState);
}

pub struct SpatialiteConnection {
  sqlite_connection: rusqlite::Connection,
  spatialite_state: *mut SpatialiteState
}

pub fn create_spatialite_connection(connection: rusqlite::Connection) -> SpatialiteConnection {
  unsafe {
    let state = spatialite_alloc_connection();
    spatialite_init_ex(connection.handle(), state, 0);
    return SpatialiteConnection {
      sqlite_connection: connection,
      spatialite_state: state
    };
  }
}

impl Drop for SpatialiteConnection {
  fn drop(&mut self) {
    unsafe {
      spatialite_cleanup_ex(self.spatialite_state);
    }
  }
}
