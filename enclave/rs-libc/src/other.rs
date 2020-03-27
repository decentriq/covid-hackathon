use std::os::raw::{c_char, c_int};
use std::ptr;
use std::process;

#[no_mangle] pub unsafe extern fn open() { unimplemented!() }
#[no_mangle] pub unsafe extern fn close() { println!("close") }
#[no_mangle] pub unsafe extern fn access() { unimplemented!() }
#[no_mangle] pub unsafe extern fn getcwd() { unimplemented!() }
#[no_mangle] pub unsafe extern fn stat64() { unimplemented!() }
#[no_mangle] pub unsafe extern fn lstat64() { unimplemented!() }
#[no_mangle] pub unsafe extern fn fstat64() { unimplemented!() }
#[no_mangle] pub unsafe extern fn ftruncate64() { unimplemented!() }
#[no_mangle] pub unsafe extern fn fcntl() { unimplemented!() }
#[no_mangle] pub unsafe extern fn read() { println!("read") }
#[no_mangle] pub unsafe extern fn write() { unimplemented!() }
#[no_mangle] pub unsafe extern fn fchmod() { unimplemented!() }
#[no_mangle] pub unsafe extern fn unlink() { unimplemented!() }
#[no_mangle] pub unsafe extern fn mkdir() { unimplemented!() }
#[no_mangle] pub unsafe extern fn rmdir() { unimplemented!() }
#[no_mangle] pub unsafe extern fn fchown() { unimplemented!() }
#[no_mangle] pub unsafe extern fn geteuid() { unimplemented!() }
#[no_mangle] pub unsafe extern fn mmap64() { unimplemented!() }
#[no_mangle] pub unsafe extern fn munmap() { unimplemented!() }
#[no_mangle] pub unsafe extern fn readlink() { unimplemented!() }
#[no_mangle] pub unsafe extern fn gettimeofday() { unimplemented!() }
#[no_mangle] pub unsafe extern fn usleep() { unimplemented!() }
#[no_mangle] pub unsafe extern fn pthread_mutexattr_init() { println!("pthread_mutexattr_init")}
#[no_mangle] pub unsafe extern fn pthread_mutexattr_destroy() { println!("pthread_mutexattr_destroy") }
#[no_mangle] pub unsafe extern fn pthread_mutexattr_settype() { println!("pthread_mutexattr_settype")}
#[no_mangle] pub unsafe extern fn pthread_mutex_trylock() { unimplemented!() }
#[no_mangle] pub unsafe extern fn lseek64() { unimplemented!() }
#[no_mangle] pub unsafe extern fn fsync() { unimplemented!() }
#[no_mangle] pub unsafe extern fn open64() { println!("open64") }
#[no_mangle] pub unsafe extern fn sysconf() { unimplemented!() }
#[no_mangle] pub unsafe extern fn getenv() { unimplemented!() }
#[no_mangle] pub unsafe extern fn utimes() { unimplemented!() }
#[no_mangle]
pub unsafe extern fn strrchr(cs: *const c_char, character: c_int) -> *const c_char {
    println!("strrchr");
    let mut found: *const c_char = ptr::null_mut();
    let c = character as u8;
    let mut i = 0;
    while *cs.offset(i) as u8 != b'\0' {
        if *cs.offset(i) as u8 == c {
            found = cs.offset(i);
        }
        i += 1;
    }
    found
}
#[no_mangle] pub unsafe extern fn localtime() { unimplemented!() }
#[no_mangle] pub unsafe extern fn qsort() { unimplemented!() }
#[no_mangle] pub unsafe extern fn __stack_chk_guard() { unimplemented!() }
#[no_mangle] pub unsafe extern fn __errno_location() { println!("__errno_location") }
#[no_mangle] pub unsafe extern fn getpid() -> u32 {
    println!("getpid");
    0
}
#[no_mangle] pub unsafe extern fn dlerror() { unimplemented!() }
#[no_mangle] pub unsafe extern fn dlopen() { unimplemented!() }
#[no_mangle] pub unsafe extern fn dlsym() { unimplemented!() }
#[no_mangle] pub unsafe extern fn dlclose() { unimplemented!() }
#[no_mangle] pub unsafe extern fn time() { println!("time") }
#[no_mangle] pub unsafe extern fn pthread_self() { println!("pthread_self") }
#[no_mangle] pub unsafe extern fn pthread_equal() -> u32 { println!("pthread_equal"); 1 }

