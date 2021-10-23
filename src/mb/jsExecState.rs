#![allow(non_camel_case_types)]

use crate::mb::{MB, jsExecState, jsValue};




impl jsExecState {
    pub fn getArg(self, argId: i32) -> jsValue {
        MB::jsArg(self, argId)
    }

}
