use std::ffi::{CStr, CString};

pub fn rustToCStr(str: &str) -> *mut i8 {
    CString::new(str).unwrap().into_raw()
}

pub fn cToRustStr<'a>(str: *const i8) -> &'a str {
    unsafe { CStr::from_ptr(str).to_str().unwrap() }
}
