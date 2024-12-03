use std::ffi::{c_char, CStr, CString};

use ewts::EwtsConverter;

/// # Safety
/// The ewts_src should be a valid pointer to the string
#[no_mangle]
pub unsafe extern "C" fn ewts_to_unicode(ewts_src: *const c_char) -> *const c_char {
    let c_str = unsafe { CStr::from_ptr(ewts_src) };
    let str_slice: &str = c_str.to_str().unwrap();
    let result = EwtsConverter::create().ewts_to_unicode(str_slice);
    let c_string = CString::new(result).expect("CString::new failed");
    c_string.into_raw()
}


/// # Safety
/// The ptr should be a pointer to the string returned from convert function
#[no_mangle]
pub unsafe extern fn free_ewts_string(ptr: *const c_char) {
    // Take the ownership back to rust and drop the owner
    let _ = CString::from_raw(ptr as *mut _);
}


