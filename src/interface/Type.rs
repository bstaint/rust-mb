#![allow(non_camel_case_types)]

pub struct Window {
    pub style: i32,
    pub parent: i32,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

pub struct Object;

pub type Netjob = isize;

pub type Webview = isize;
#[derive(Clone, Copy, Debug)]
pub struct jsExecState {
    value: isize,
}
#[derive(Clone, Copy, Debug)]
pub struct jsValue {
    value: isize,
}

#[derive(Debug)]
pub struct MB {
    pub webview: Webview,
    pub url: String,
}

impl Default for Window {
    fn default() -> Self {
        Window {
            style: 0,
            parent: 0,
            x: 0,
            y: 0,
            width: 800,
            height: 600,
        }
    }
}
