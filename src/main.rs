#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(improper_ctypes)]

use std::os::raw::{c_char, c_int};
use std::ffi::CString;

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/bindings.rs"));

fn main()
{
    println!("HELLO XENO!");

    unsafe {
        let mut q : RT_QUEUE = RT_QUEUE{handle: 0};
        let q_ptr : *mut RT_QUEUE = &mut q;

        let name : *const c_char  = CString::new("MYNAME!").expect("CString::new failed").as_ptr();

        let pool_size : size_t = 0;
        let q_limit : size_t = 0;
        let mode : c_int = 0;

        rt_queue_create(q_ptr, name, pool_size, q_limit, mode);
    }
}