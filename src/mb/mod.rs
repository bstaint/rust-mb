

pub mod Func;
pub mod util;
use crate::interface::Type::*;

#[derive(Debug)]
pub struct MB {
    pub webview: Webview,
    pub url: String,
}