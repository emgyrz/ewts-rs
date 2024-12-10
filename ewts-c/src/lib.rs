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
/// # Safety
/// Frees `EwtsConverter`.
/// Gets pointer returned fron `create_ewts_converter()` fn
///
#[no_mangle]
pub unsafe extern "C" fn free_ewts_converter(ewts_converter_ptr: usize) {
    let _ = Box::from_raw(ewts_converter_ptr as *mut EwtsConverter);
}

///
/// # Panics
///
/// Panics if arguments are wrong
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

/// # Safety
/// The ptr should be a pointer to the string returned from convert function
#[no_mangle]
pub unsafe extern "C" fn free_ewts_string(ptr: *const c_char) {
    // Take the ownership back to rust and drop the owner
    let _ = CString::from_raw(ptr as *mut _);
}
