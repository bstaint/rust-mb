#![allow(non_camel_case_types)]

use crate::interface::Type::*;

impl jsExecState {
    pub fn getArg(self, argId: i32) -> jsValue {
        MB::jsArg(self, argId)
    }

}
