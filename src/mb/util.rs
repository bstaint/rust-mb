#![allow(non_snake_case)]

use std::ffi::{CStr, CString};

pub fn rustToCStr(str: &str) -> *mut i8 {
    CString::new(str).unwrap().into_raw()
}

pub fn rustToCStrW(str: &str) -> *const u16 {

    let path = format!("{}\0", str);
    path.encode_utf16().collect::<Vec<_>>().as_ptr()

}

pub fn cToRustStr<'a>(str: *const i8) -> &'a str {
    unsafe { CStr::from_ptr(str).to_str().unwrap() }
}
