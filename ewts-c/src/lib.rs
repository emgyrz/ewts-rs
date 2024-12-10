//! 
//! A little wrapper around core rust ewts conversion library, usable in C/C++/Cython code.
//! Or anywhere a C-code can be called.
//! See [example](https://github.com/emgyrz/ewts-rs/tree/master/ewts-c/examples)
//! and [test code](https://github.com/emgyrz/ewts-rs/blob/master/bench/cpp_bench.cpp).
//! Also see for details [here](https://github.com/emgyrz/ewts-rs/tree/master/ewts-c).
//!
//! It is part of set of apps/libs called **ewts-rs**.
//!
//! See more [here](https://github.com/emgyrz/ewts-rs)
//!
//! # Example
//! ```cpp
//! // some C++ file
//! // ...
//! uintptr_t converter_ptr = create_ewts_converter();
//!
//! const char * converted_str = ewts_to_unicode(converter_ptr, "rgyu ");
//! // "རྒྱུ་"
//! // ...
//! ```
//!
use std::ffi::{c_char, CStr, CString};

use ewts::EwtsConverter;

///
/// Creates `EwtsConverter` and returns its pointer
/// 
#[no_mangle]
pub extern "C" fn create_ewts_converter() -> usize {
    let wrapper = EwtsConverter::create();
    Box::into_raw(Box::new(wrapper)) as usize
}

///
/// Frees `EwtsConverter`.
/// Gets pointer to EwtsConverter instance
///
/// # Safety
/// The `ewts_converter_ptr` should be pointer returned from `create_ewts_converter()` fn.
///
#[no_mangle]
pub unsafe extern "C" fn free_ewts_converter(ewts_converter_ptr: usize) {
    let _ = Box::from_raw(ewts_converter_ptr as *mut EwtsConverter);
}

///
/// Converts EWTS-string to Tibetan unicode string.
///
/// # Example
/// ```cpp
/// // some C++ file
/// uintptr_t converter_ptr = create_ewts_converter();
/// const char * converted_str = ewts_to_unicode(converter_ptr, "rgyu ");
/// // "རྒྱུ་"
/// ```
///
/// # Safety
/// The `ewts_converter_ptr` should be pointer returned from `create_ewts_converter()` fn.
/// And `ewts_src` should be a valid pointer to the string
///
#[no_mangle]
pub unsafe extern "C" fn ewts_to_unicode(ewts_converter_ptr: usize, ewts_src: *const c_char) -> *const c_char {
    let c_str = CStr::from_ptr(ewts_src);
    let str_slice: &str = c_str.to_str().unwrap();
    let converter = (ewts_converter_ptr as *mut EwtsConverter).as_ref().unwrap();
    let result = converter.ewts_to_unicode(str_slice);
    let c_string = CString::new(result).unwrap();
    c_string.into_raw()
}

/// As a precaution
///
/// # Safety
/// The ptr should be a pointer to the string returned from convert function
#[no_mangle]
pub unsafe extern "C" fn free_ewts_string(ptr: *const c_char) {
    let _ = CString::from_raw(ptr as *mut _);
}
