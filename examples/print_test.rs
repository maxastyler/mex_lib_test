#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

extern crate libc;
extern crate mex_lib_test;

use std::ffi::CString;
use mex_lib_test::bindings::*;
use mex_lib_test::sum_of_squares;
use libc::{c_double, c_long};

#[no_mangle]
pub extern "C" fn mexFunction(nlhs: ::std::os::raw::c_int, plhs: *mut *mut mxArray,
                              nrhs: ::std::os::raw::c_int, prhs: *mut *mut mxArray){
    let id = CString::new("MATLAB:yprime:divideByZero").unwrap();
    let msg = CString::new("Division by zero!\n").unwrap();
    let a: Vec<i32> = (0..1000).collect();
    let res = CString::new(format!("The sum of the squares is: {}", sum_of_squares(&a))).unwrap();
    unsafe {
        mexPrintf(res.as_ptr());
    }
}


