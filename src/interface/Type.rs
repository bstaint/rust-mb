pub struct Window {
    pub style: i32,
    pub parent: i32,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

pub type Webview = isize;
pub type jsExecState = isize;
pub type jsValue = isize;
pub type HWND = isize;
pub type WPARAM = isize;
pub type LPARAM = isize;

#[derive(Debug)]
pub struct MB {
    pub webview: Webview,
    pub url: String,
}