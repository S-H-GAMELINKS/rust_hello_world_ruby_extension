#![allow(non_snake_case)]

extern crate libc;
use std::ffi::CString;

extern {
    fn rb_define_global_function(name: *const libc::c_char, func: extern fn(), argc: libc::c_int);
}

extern fn hello_world() {
    println!("Hello, world!");
}

#[no_mangle]
pub extern fn Init_librust_hello_world() {
    let c_func_name = CString::new("hello_world").unwrap();
    let argc = 0;

    unsafe {
        rb_define_global_function(c_func_name.as_ptr(), hello_world, argc);
    }
}