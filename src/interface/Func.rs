use super::Type::*;

pub type Initialize = fn();
pub type CreateWebWindow = fn(i32, i32, i32, i32, i32, i32) -> Webview;
pub type ShowWindow = fn(Webview, bool);
pub type MoveToCenter = fn(Webview);
pub type EnableHighDPISupport = fn();
pub type RunMessageLoop = fn();
pub type SetWindowTitle = fn(Webview, *mut i8);
pub type LoadUrl = fn(Webview, *mut i8);
pub type OnWindowDestroy = fn(Webview, fn());
pub type JsBindFunction = fn(*mut i8, fn(es: jsExecState), i32);
pub type GetWindowHandle = fn(webview: Webview) -> HWND;
pub type FireWindowsMessage =
    fn(webview: Webview, hWnd: HWND, message: i32, wParam: WPARAM, lParam: LPARAM) -> bool;
    
pub type jsArg = fn(es: jsExecState, argId: i32) -> jsValue;
pub type jsToString = fn(es: jsExecState, value: jsValue) -> *const i8;
pub type jsString = fn(es: jsExecState, str: *const u8) -> String;
