extern crate rs_libc;
extern crate rs_libc_decentriq;
use libtest_sys as ffi;
use std::os::raw::c_char;
use std::ffi::CStr;

fn main() {
    unsafe { ffi::test_function(32); }
    unsafe { ffi::test_file(); }
    unsafe { ffi::test_pthread(); }
}

#[no_mangle]
pub extern fn rust_test_function(test: i32) {
    println!("Calling from C: {}", test);
}

#[no_mangle]
pub extern fn logger(message: *const c_char) {
    let c_str: &CStr = unsafe { CStr::from_ptr(message) };
    println!("Log: {}", c_str.to_str().unwrap());
}
