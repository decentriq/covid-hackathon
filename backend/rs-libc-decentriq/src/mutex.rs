use std::os::fortanix_sgx;
use std::os::raw::c_char;
use std::ffi::{CStr, c_void};
use std::fmt;
use std::thread;
use std::sync::{Arc, Mutex, MutexGuard};
use std::ptr;

pub struct pthread_mutex_internal {
    owner: thread::ThreadId,
    recursive: bool,
    count: i32,
    handle: *mut c_void,
    lock: *mut c_void
}

#[repr(C)]
pub struct pthread_mutex_t { 
    init: u32,
    internal: *mut pthread_mutex_internal
}

#[repr(C)]
pub struct pthread_mutexattr_t { 
    flags: i32
}

#[no_mangle]
pub unsafe extern fn pthread_mutex_init(mutex: *mut pthread_mutex_t, attributes: *const pthread_mutexattr_t) -> i32 {
    println!("begin pthread_mutex_init");
    let mut recursive = false;
    if (attributes != ptr::null()) {
        println!("init recursive mutex");
        recursive = true;
    }
    let internal = Box::into_raw(Box::new(pthread_mutex_internal {
        owner: thread::current().id(),
        recursive,
        count: 0,
        handle: Box::into_raw(Box::new(Mutex::new(0))) as *mut c_void,
        lock: ptr::null_mut()
    }));
    let mutex_original = &mut *mutex;
    mutex_original.init = 0x1337;
    mutex_original.internal = internal;
    println!("end pthread_mutex_init");
    0
}

#[no_mangle]
pub unsafe extern fn pthread_mutex_destroy(mutex: *mut pthread_mutex_t) -> i32 {
    println!("begin pthread_mutex_destroy");
    let mutex_original = &mut *mutex;
    let mutex_internal = &mut *mutex_original.internal;
    let handle = &mut *(mutex_internal.handle as *mut Mutex<i32>);
    println!("end pthread_mutex_destroy");
    0
}

#[no_mangle]
pub unsafe extern fn pthread_mutex_lock(mutex: *mut pthread_mutex_t) -> i32 {
    println!("begin pthread_mutex_lock in {:?}", thread::current().id());
    let mutex_original = &mut *mutex;
    if mutex_original.init != 0x1337 {
        let internal = Box::into_raw(Box::new(pthread_mutex_internal {
            owner: thread::current().id(),
            recursive: false,
            count: 0,
            handle: Box::into_raw(Box::new(Mutex::new(0))) as *mut c_void,
            lock: ptr::null_mut()
        }));
        mutex_original.internal = internal;
    }
    let mutex_internal = &mut *mutex_original.internal;
    if mutex_internal.recursive && mutex_internal.owner == thread::current().id() && mutex_internal.lock != ptr::null_mut() {
        mutex_internal.count += 1;
        println!("recursive mutex and owner same, don't lock");
        return 0;
    }
    let handle = &mut *(mutex_internal.handle as *mut Mutex<i32>);
    // Push the MutexGuard on the heap so it doesn't get dropped at the
    // end of the scope of this function, keeping the mutex locked
    mutex_internal.lock = Box::into_raw(Box::new(handle.lock().unwrap())) as *mut c_void;
    println!("end pthread_mutex_lock in {:?}", thread::current().id());
    0
}

#[no_mangle]
pub unsafe extern fn pthread_mutex_unlock(mutex: *mut pthread_mutex_t) -> i32 {
    println!("begin pthread_mutex_unlock in {:?}", thread::current().id());
    let mutex_original = &mut *mutex;
    let mutex_internal = &mut *mutex_original.internal;
    if mutex_internal.recursive && mutex_internal.owner == thread::current().id() && mutex_internal.lock != ptr::null_mut() {
        mutex_internal.count -= 1;
        if mutex_internal.count != 0 {
            println!("recursive mutex and owner same and has still locks, don't unlock");
            return 0;
        }
    }
    // Get back the MutexGuard from the heap so that it gets dropped
    // when leaving this function scope, unlocking the mutex
    let _lock_original = Box::from_raw(mutex_internal.lock as *mut MutexGuard<i32>);
    mutex_internal.lock = ptr::null_mut() as *mut c_void;
    println!("end pthread_mutex_unlock in {:?}", thread::current().id());
    0
}
