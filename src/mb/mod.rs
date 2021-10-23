#![allow(non_snake_case)]


pub mod mbAPI;
pub mod util;

// #![allow(non_camel_case_types)]

pub struct mbWindow {
    pub style: mbWindowType,
    pub parent: i32,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

pub enum mbWindowType {
    WKE_WINDOW_TYPE_POPUP = 0,
    WKE_WINDOW_TYPE_TRANSPARENT = 1,
    WKE_WINDOW_TYPE_CONTROL = 2,
}


#[derive(Clone, Copy, Debug)]
pub struct Netjob {
    value: isize,
}

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

impl Default for mbWindow {
    fn default() -> Self {
        mbWindow {
            style: mbWindowType::WKE_WINDOW_TYPE_POPUP,
            parent: 0,
            x: 0,
            y: 0,
            width: 800,
            height: 600,
        }
    }
}
