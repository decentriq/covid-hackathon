use std::os::fortanix_sgx;
use std::os::raw::c_char;
use std::ffi::{CStr, c_void};
use std::fmt;
use std::thread;

#[repr(C)]
pub struct pthread_attr_t {
    flags: i32,
    stacksize: i32
}

pub struct pthread_internal {
    attributes: *const pthread_attr_t,
    routine: extern fn(*mut c_void)->*mut c_void,
    argument: *mut c_void,
    handle: *mut c_void
}

#[repr(C)]
pub struct pthread_t { 
    internal: *mut pthread_internal
}

impl fmt::Display for pthread_t {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "pthread")
    }
}

#[no_mangle]
pub unsafe extern fn sleep(seconds: u32) -> u32 {
    println!("sleep");
    0
}

struct MyVoid(*mut std::ffi::c_void);
unsafe impl Send for MyVoid {}

#[no_mangle]
pub unsafe extern fn pthread_create(thread: *mut pthread_t, attributes: *const pthread_attr_t, routine: extern fn(*mut c_void)->*mut c_void, argument: *mut c_void) -> i32 {
    println!("begin pthread_create");
    let my_void = MyVoid(argument);
    let handle = thread::spawn(move || {
        MyVoid(routine(my_void.0))
    });
    let internal = Box::into_raw(Box::new(pthread_internal {
        attributes,
        routine,
        argument,
        handle: Box::into_raw(Box::new(Some(handle))) as *mut c_void
    }));
    let thread_original = &mut *thread;
    thread_original.internal = internal;
    println!("end pthread_create");
    0
}

#[no_mangle]
pub unsafe extern fn pthread_join(thread: pthread_t, return_value: *mut *mut c_void) -> i32 {
    println!("begin pthread_join");
    let thread_internal = &mut *thread.internal;
    let handle = &mut *(thread_internal.handle as *mut Option<thread::JoinHandle<MyVoid>>);
    let result = handle.take().unwrap().join();
    println!("end pthread_join");
    0
}
