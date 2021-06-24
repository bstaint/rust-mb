#![allow(non_camel_case_types)]

use winapi::shared::{
    minwindef::{LPARAM, WPARAM},
    windef::HWND,
};

use super::Type::*;

pub type Initialize = fn();
pub type CreateWebWindow = fn(i32, i32, i32, i32, i32, i32) -> Webview;
pub type ShowWindow = fn(Webview, bool);
pub type MoveToCenter = fn(Webview);
pub type EnableHighDPISupport = fn();
pub type RunMessageLoop = fn();
pub type SetWindowTitle = fn(Webview, *mut i8);
pub type SetDebugConfig = fn(Webview, *mut i8, *mut i8);

pub type LoadUrl = fn(Webview, *mut i8);
pub type LoadHTML = fn(Webview, *mut i8);
pub type LoadFile = fn(Webview, *mut i8);

pub type OnDocumentReady = fn(Webview, fn());
pub type OnLoadingFinish = fn(Webview, fn());
pub type OnWindowDestroy = fn(Webview, fn());

pub type OnLoadUrlBegin = fn(Webview, LoadUrlBeginCallback, i32);
pub type OnLoadUrlEnd = fn(Webview, LoadUrlEndCallback, i32);

pub type IsInitialize = fn() -> bool;

pub type GetZoomFactor = fn(Webview) -> f32;
pub type SetZoomFactor = fn(Webview, f32) -> f32;

pub type JsBindFunction = fn(*mut i8, fn(es: jsExecState) -> jsValue, i32);
pub type RunJS = fn(Webview, *const i8) -> jsValue;
pub type GetWindowHandle = fn(webview: Webview) -> HWND;
pub type FireWindowsMessage =
    fn(webview: Webview, hWnd: HWND, message: i32, wParam: WPARAM, lParam: LPARAM) -> bool;

pub type ShowDevtools = fn(webview: Webview, path: *const u16, callback: i32, param: i32);
pub type GetWebViewForCurrentContext = fn() -> Webview;
pub type GetSource = fn(webview: Webview) -> *const i8;
pub type GetURL = fn(webview: Webview) -> *const i8;
pub type GetTitle=fn(webview: Webview)->*const i8;

pub type GlobalExec = fn(webview: Webview) -> jsExecState;

//http hook
pub type NetSetData = fn(Netjob, *const i8, i32);
pub type NetHookRequest = fn(Netjob);

//callbacks
pub type LoadUrlEndCallback = fn(Webview, i32, *const i8, Netjob, *const i8, i32);
pub type LoadUrlBeginCallback = fn(Webview, i32, *const i8, Netjob);

//js
pub type jsArg = fn(es: jsExecState, argId: i32) -> jsValue;

pub type jsToString = fn(es: jsExecState, value: jsValue) -> *const i8;
pub type jsToInt = fn(es: jsExecState, value: jsValue) -> i32;
pub type jsToFloat = fn(es: jsExecState, value: jsValue) -> f32;
pub type jsToBoolean = fn(es: jsExecState, value: jsValue) -> bool;

pub type jsInt = fn(n: i32) -> jsValue;
pub type jsString = fn(es: jsExecState, str: *const i8) -> jsValue;
pub type jsEmptyObject = fn(es: jsExecState) -> jsValue;
pub type jsEmptyArray = fn(es: jsExecState) -> jsValue;

pub type jsUndefined = fn() -> jsValue;
pub type jsNull = fn() -> jsValue;
pub type jsTrue = fn() -> jsValue;
pub type jsFalse = fn() -> jsValue;

pub type jsGetAt = fn(es: jsExecState, object: jsValue, index: i32) -> jsValue;
pub type jsSetAt = fn(es: jsExecState, object: jsValue, index: i32, v: jsValue);

pub type jsGet = fn(es: jsExecState, object: jsValue, prop: *mut i8) -> jsValue;
pub type jsSet = fn(es: jsExecState, object: jsValue, prop: *mut i8, value: jsValue);
pub type jsGC = fn();

pub type jsGetGlobal = fn(es: jsExecState, prop: *const i8) -> jsValue;
