#![allow(non_camel_case_types)]

use crate::mb::{MB, Netjob};


impl Netjob {
    pub fn HookRequest(self) {
        MB::NetHookRequest(self);
    }

    pub fn SetData(self, buf: *const i8, len: i32) {
        MB::NetSetData(self, buf, len);
    }
}
