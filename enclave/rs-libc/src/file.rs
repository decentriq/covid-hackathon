use std::os::raw::c_char;
use std::ffi::{CStr, c_void};
use std::fmt;

#[repr(C)]
pub struct FILE { 
    path: *const c_char,
    mode: *const c_char,
}

impl fmt::Display for FILE {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let path: &CStr = unsafe { CStr::from_ptr(self.path) };
        let mode: &CStr = unsafe { CStr::from_ptr(self.mode) };
        write!(f, "File path={}, mode={}", path.to_str().unwrap(), mode.to_str().unwrap())
    }
}

#[no_mangle]
pub unsafe extern fn fopen(path: *const c_char, mode: *const c_char) -> *mut FILE {
    println!("fopen");
    Box::into_raw(Box::new(FILE {
        path,
        mode,
    }))
}

#[no_mangle]
pub unsafe extern fn fprintf(file: *mut FILE, format: *const c_char, ...) -> i32 {
    println!("fprintf");
    println!("{}", &mut *file);
    0
}

#[no_mangle]
pub unsafe extern "C" fn fputs(string: *const c_char, file: *mut FILE) -> i32 {
    println!("fputs");
    println!("{}", &mut *file);
    0
}

#[no_mangle]
pub unsafe extern "C" fn puts(string: *const c_char) -> i32 {
    println!("puts");
    let message: &CStr = CStr::from_ptr(string);
    println!("{}", message.to_str().unwrap());
    0
}

#[no_mangle]
pub unsafe extern "C" fn fwrite(pointer: *const c_void, size: usize, count: usize, file: *mut FILE) -> usize {
    println!("fwrite");
    println!("{}", &mut *file);
    0
}

#[no_mangle]
pub unsafe extern "C" fn fread(pointer: *mut c_void, size: usize, count: usize, file: *mut FILE) -> usize {
    println!("fread");
    println!("{}", &mut *file);
    0
}

#[no_mangle]
pub unsafe extern "C" fn fclose(file: *mut FILE) -> i32 {
    println!("fclose");
    println!("{}", &mut *file);
    0
}
