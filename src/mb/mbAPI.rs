#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
use super::{MB, Netjob, Webview, mbWindow as Window, jsExecState, jsValue, util::*};

use crate::interface::*;

use lazy_static::lazy_static;
use libloading::{Library, Symbol};
use winapi::shared::{
    minwindef::{LPARAM, WPARAM},
    windef::HWND,
};

use std::fs;
lazy_static! {
    static ref nodeDll: Library = unsafe {
        Library::new("./node.dll").expect("node.dll不存在,请前往 https://miniblink.net/ 下载")
    };
}

impl MB {
    pub fn new() -> MB {
        MB {
            webview: 0,
            url: String::new(),
        }
    }
    ///初始化miniblink
    pub fn Initialize() {
        let lib = &nodeDll;
        let wkeInitialize: Symbol<Initialize> = unsafe { lib.get(b"wkeInitialize").unwrap() };

        wkeInitialize();
    }

    ///创建一个窗口
    pub fn CreateWebWindow(&mut self, window: Window) -> &mut MB {
        let lib = &nodeDll;
        let wkeCreateWebWindow: Symbol<CreateWebWindow> =
            unsafe { lib.get(b"wkeCreateWebWindow").unwrap() };
        let webview = wkeCreateWebWindow(
            window.style as i32,
            window.parent,
            window.x,
            window.y,
            window.width,
            window.height,
        );
        self.webview = webview;
        self
    }

    ///显示窗口
    pub fn ShowWindow(&mut self) -> &mut MB {
        let lib = &nodeDll;
        let wkeShowWindow: Symbol<ShowWindow> = unsafe { lib.get(b"wkeShowWindow").unwrap() };

        wkeShowWindow(self.webview, true);
        self
    }

    ///窗口居中
    pub fn MoveToCenter(&mut self) -> &mut MB {
        let lib = &nodeDll;
        let wkeMoveToCenter: Symbol<MoveToCenter> = unsafe { lib.get(b"wkeMoveToCenter").unwrap() };
        wkeMoveToCenter(self.webview);
        self
    }

    ///开启高dpi支持
    pub fn EnableHighDPISupport() {
        let lib = &nodeDll;
        let wkeEnableHighDPISupport: Symbol<EnableHighDPISupport> =
            unsafe { lib.get(b"wkeEnableHighDPISupport").unwrap() };
        wkeEnableHighDPISupport();
    }

    ///开启消息循环
    pub fn RunMessageLoop() {
        let lib = &nodeDll;
        let wkeRunMessageLoop: Symbol<RunMessageLoop> =
            unsafe { lib.get(b"wkeRunMessageLoop").unwrap() };
        wkeRunMessageLoop();
    }

    ///设置窗口标题
    pub fn SetWindowTitle(&mut self, title: &str) -> &mut MB {
        let lib = &nodeDll;
        let wkeSetWindowTitle: Symbol<SetWindowTitle> =
            unsafe { lib.get(b"wkeSetWindowTitle").unwrap() };
        let c_title = rustToCStr(title);
        wkeSetWindowTitle(self.webview, c_title);
        self
    }

    ///# 开启实验性选项
    ///### 开启Nodejs(只支持32位)
    /// debugString:"enableNodejs"
    ///
    /// params:"1"
    ///
    ///```
    ///let mut mb=MB::new();
    ///mb.SetDebugConfig("enableNodejs", "1");
    ///```
    pub fn SetDebugConfig(&mut self, debugString: &str, params: &str) -> &mut MB {
        let lib = &nodeDll;
        let wkeSetDebugConfig: Symbol<SetDebugConfig> =
            unsafe { lib.get(b"wkeSetDebugConfig").unwrap() };
        let c_debug = rustToCStr(debugString);
        let c_params = rustToCStr(params);
        wkeSetDebugConfig(self.webview, c_debug, c_params);
        self
    }

    ///加载URL
    pub fn LoadUrl(&mut self, url: &str) -> &mut MB {
        let lib = &nodeDll;
        let wkeLoadURL: Symbol<LoadUrl> = unsafe { lib.get(b"wkeLoadURL").unwrap() };
        self.url = String::from(url);

        let url = rustToCStr(url);
        wkeLoadURL(self.webview, url);
        self
    }

    ///加载HTML
    pub fn LoadHTML(&mut self, html: &str) -> &mut MB {
        let lib = &nodeDll;
        let wkeLoadHTML: Symbol<LoadHTML> = unsafe { lib.get(b"wkeLoadHTML").unwrap() };

        let html = rustToCStr(html);
        wkeLoadHTML(self.webview, html);
        self
    }

    ///加载HTML文件
    pub fn LoadFile(&mut self, filename: &str) -> &mut MB {
        let lib = &nodeDll;
        let wkeLoadFile: Symbol<LoadFile> = unsafe { lib.get(b"wkeLoadFile").unwrap() };

        let filename = rustToCStr(filename);
        wkeLoadFile(self.webview, filename);
        self
    }
    ///是否初始化完成
    pub fn IsInitialize() -> bool {
        let lib = &nodeDll;
        let wkeIsInitialize: Symbol<IsInitialize> = unsafe { lib.get(b"wkeIsInitialize").unwrap() };
        wkeIsInitialize()
    }

    ///获取缩放系数
    pub fn GetZoomFactor(&mut self) -> f32 {
        let lib = &nodeDll;
        let wkeGetZoomFactor: Symbol<GetZoomFactor> =
            unsafe { lib.get(b"wkeGetZoomFactor").unwrap() };
        wkeGetZoomFactor(self.webview)
    }
    ///设置缩放系数
    pub fn SetZoomFactor(&mut self, zoom: f32) -> &mut MB {
        let lib = &nodeDll;
        let wkeSetZoomFactor: Symbol<SetZoomFactor> =
            unsafe { lib.get(b"wkeSetZoomFactor").unwrap() };
        wkeSetZoomFactor(self.webview, zoom);
        self
    }

    ///dom加载完成回调
    pub fn OnDocumentReady(&mut self, callback: fn()) -> &mut MB {
        let lib = &nodeDll;
        let wkeOnDocumentReady: Symbol<OnDocumentReady> =
            unsafe { lib.get(b"wkeOnDocumentReady").unwrap() };

        wkeOnDocumentReady(self.webview, callback);
        self
    }

    ///网页加载完成回调
    pub fn OnLoadingFinish(&mut self, callback: fn()) -> &mut MB {
        let lib = &nodeDll;
        let wkeOnLoadingFinish: Symbol<OnLoadingFinish> =
            unsafe { lib.get(b"wkeOnLoadingFinish").unwrap() };

        wkeOnLoadingFinish(self.webview, callback);
        self
    }

    ///Url开始加载回调
    pub fn OnLoadUrlBegin(&mut self, callback: LoadUrlBeginCallback, param: i32) -> &mut MB {
        let lib = &nodeDll;
        let wkeOnLoadUrlBegin: Symbol<OnLoadUrlBegin> =
            unsafe { lib.get(b"wkeOnLoadUrlBegin").unwrap() };

        wkeOnLoadUrlBegin(self.webview, callback, param);
        self
    }

    ///Url加载完成回调
    pub fn OnLoadUrlEnd(&mut self, callback: LoadUrlEndCallback, param: i32) -> &mut MB {
        let lib = &nodeDll;
        let wkeOnLoadUrlEnd: Symbol<OnLoadUrlEnd> = unsafe { lib.get(b"wkeOnLoadUrlEnd").unwrap() };

        wkeOnLoadUrlEnd(self.webview, callback, param);
        self
    }

    ///窗体销毁回调
    pub fn OnWindowDestroy(&mut self, callback: fn()) -> &mut MB {
        let lib = &nodeDll;
        let wkeOnWindowDestroy: Symbol<OnWindowDestroy> =
            unsafe { lib.get(b"wkeOnWindowDestroy").unwrap() };

        wkeOnWindowDestroy(self.webview, callback);
        self
    }

    ///设置网络请求数据
    pub fn NetSetData(jobptr: Netjob, buf: *const i8, len: i32) {
        let lib = &nodeDll;
        let wkeNetSetData: Symbol<NetSetData> = unsafe { lib.get(b"wkeNetSetData").unwrap() };

        wkeNetSetData(jobptr, buf, len);
    }

    ///拦截网络请求
    pub fn NetHookRequest(jobptr: Netjob) {
        let lib = &nodeDll;
        let wkeNetHookRequest: Symbol<NetHookRequest> =
            unsafe { lib.get(b"wkeNetHookRequest").unwrap() };

        wkeNetHookRequest(jobptr);
    }

    ///JS绑定方法
    pub fn JsBindFunction(msg: &str, callback: fn(es: jsExecState) -> jsValue, args: i32) {
        let lib = &nodeDll;
        let wkeJsBindFunction: Symbol<JsBindFunction> =
            unsafe { lib.get(b"wkeJsBindFunction").unwrap() };
        let c_msg = rustToCStr(msg);
        wkeJsBindFunction(c_msg, callback, args);
    }

    ///执行JS
    pub fn RunJS(&mut self, script: &str) -> jsValue {
        let lib = &nodeDll;
        let wkeRunJS: Symbol<RunJS> = unsafe { lib.get(b"wkeRunJS").unwrap() };

        let script = rustToCStr(script);
        wkeRunJS(self.webview, script)
    }
    /// 打开开发者工具
    pub fn ShowDevtools(&mut self, path: &str) {
        let lib = &nodeDll;
        let ShowDevtools: Symbol<ShowDevtools> = unsafe { lib.get(b"wkeShowDevtools").unwrap() };
        let path = rustToCStrW(path);
        ShowDevtools(self.webview, path as *mut u16, 0, 0);
    }

    /// 获取网页HTML
    pub fn GetSource<'a>(&mut self) -> &'a str {
        let lib = &nodeDll;
        let wkeGetSource: Symbol<GetSource> = unsafe { lib.get(b"wkeGetSource").unwrap() };

        let source = wkeGetSource(self.webview);
        cToRustStr(source)
    }

    /// 获取当前线程 Webview
    pub fn GetWebViewForCurrentContext<'a>() -> Webview {
        let lib = &nodeDll;
        let GetWebViewForCurrentContext: Symbol<GetWebViewForCurrentContext> =
            unsafe { lib.get(b"wkeGetWebViewForCurrentContext").unwrap() };

        GetWebViewForCurrentContext()
    }

    /// 获取当前的url
    pub fn GetURL<'a>(&mut self) -> &'a str {
        let lib = &nodeDll;
        let wkeGetURL: Symbol<GetURL> = unsafe { lib.get(b"wkeGetURL").unwrap() };

        let url = wkeGetURL(self.webview);
        cToRustStr(url)
    }

    /// 获取当前的标题
    pub fn GetTitle<'a>(&mut self) -> &'a str {
        let lib = &nodeDll;
        let wkeGetTitle: Symbol<GetTitle> = unsafe { lib.get(b"wkeGetTitle").unwrap() };

        let title = wkeGetTitle(self.webview);
        cToRustStr(title)
    }

    /// 获取当前的MB
    pub fn GlobalExec(&mut self) -> jsExecState {
        let lib = &nodeDll;
        let wkeGlobalExec: Symbol<GlobalExec> = unsafe { lib.get(b"wkeGlobalExec").unwrap() };

        wkeGlobalExec(self.webview)
    }

    /// 强制js执行垃圾回收
    pub fn GC() {
        let lib = &nodeDll;
        let jsGC: Symbol<jsGC> = unsafe { lib.get(b"jsGC").unwrap() };

        jsGC()
    }

    ///获取传入参数
    pub fn jsArg(es: jsExecState, agrId: i32) -> jsValue {
        let lib = &nodeDll;
        let jsArg: Symbol<jsArg> = unsafe { lib.get(b"jsArg").unwrap() };

        jsArg(es, agrId)
    }

    ///转换成String  
    pub fn jsToString<'a>(es: jsExecState, value: jsValue) -> &'a str {
        let lib = &nodeDll;
        let jsToString: Symbol<jsToString> = unsafe { lib.get(b"jsToString").unwrap() };

        let msg = jsToString(es, value);

        cToRustStr(msg)
    }
    ///转换成i32  
    pub fn jsToInt(es: jsExecState, value: jsValue) -> i32 {
        let lib = &nodeDll;
        let jsToInt: Symbol<jsToInt> = unsafe { lib.get(b"jsToInt").unwrap() };

        jsToInt(es, value)
    }
    ///转换成f32  
    pub fn jsToFloat(es: jsExecState, value: jsValue) -> f32 {
        let lib = &nodeDll;
        let jsToFloat: Symbol<jsToFloat> = unsafe { lib.get(b"jsToFloat").unwrap() };

        jsToFloat(es, value)
    }

    ///转换成bool  
    pub fn jsToBoolean(es: jsExecState, value: jsValue) -> bool {
        let lib = &nodeDll;
        let jsToBoolean: Symbol<jsToBoolean> = unsafe { lib.get(b"jsToBoolean").unwrap() };

        jsToBoolean(es, value)
    }

    pub fn jsInt(n: i32) -> jsValue {
        let lib = &nodeDll;
        let jsInt: Symbol<jsInt> = unsafe { lib.get(b"jsInt").unwrap() };

        jsInt(n)
    }

    pub fn jsString(es: jsExecState, str: &str) -> jsValue {
        let lib = &nodeDll;
        let jsString: Symbol<jsString> = unsafe { lib.get(b"jsString").unwrap() };

        let str = rustToCStr(str);
        jsString(es, str)
    }
    pub fn jsUndefined() -> jsValue {
        let lib = &nodeDll;
        let jsUndefined: Symbol<jsUndefined> = unsafe { lib.get(b"jsUndefined").unwrap() };

        jsUndefined()
    }
    pub fn jsNull() -> jsValue {
        let lib = &nodeDll;
        let jsNull: Symbol<jsNull> = unsafe { lib.get(b"jsNull").unwrap() };

        jsNull()
    }

    pub fn jsTrue() -> jsValue {
        let lib = &nodeDll;
        let jsTrue: Symbol<jsTrue> = unsafe { lib.get(b"jsTrue").unwrap() };

        jsTrue()
    }
    pub fn jsFalse() -> jsValue {
        let lib = &nodeDll;
        let jsFalse: Symbol<jsFalse> = unsafe { lib.get(b"jsFalse").unwrap() };

        jsFalse()
    }

    pub fn jsEmptyObject(es: jsExecState) -> jsValue {
        let lib = &nodeDll;
        let jsEmptyObject: Symbol<jsEmptyObject> = unsafe { lib.get(b"jsEmptyObject").unwrap() };

        jsEmptyObject(es)
    }

    pub fn jsGet(es: jsExecState, object: jsValue, prop: &str) -> jsValue {
        let lib = &nodeDll;
        let jsGet: Symbol<jsGet> = unsafe { lib.get(b"jsGet").unwrap() };
        let msg = rustToCStr(prop);
        jsGet(es, object, msg)
    }

    pub fn jsSet(es: jsExecState, object: jsValue, prop: &str, value: jsValue) {
        let lib = &nodeDll;
        let jsSet: Symbol<jsSet> = unsafe { lib.get(b"jsSet").unwrap() };
        let msg = rustToCStr(prop);
        jsSet(es, object, msg, value);
    }

    pub fn jsEmptyArray(es: jsExecState) -> jsValue {
        let lib = &nodeDll;
        let jsEmptyArray: Symbol<jsEmptyArray> = unsafe { lib.get(b"jsEmptyArray").unwrap() };

        jsEmptyArray(es)
    }

    pub fn jsGetAt(es: jsExecState, object: jsValue, index: i32) -> jsValue {
        let lib = &nodeDll;
        let jsGetAt: Symbol<jsGetAt> = unsafe { lib.get(b"jsGetAt").unwrap() };

        jsGetAt(es, object, index)
    }

    pub fn jsSetAt(es: jsExecState, object: jsValue, index: i32, value: jsValue) {
        let lib = &nodeDll;
        let jsSetAt: Symbol<jsSetAt> = unsafe { lib.get(b"jsSetAt").unwrap() };
        jsSetAt(es, object, index, value);
    }

    pub fn jsGetGlobal(es: jsExecState, prop: &str) -> jsValue {
        let lib = &nodeDll;
        let jsGetGlobal: Symbol<jsGetGlobal> = unsafe { lib.get(b"jsGetGlobal").unwrap() };
        let prop = rustToCStr(prop);
        jsGetGlobal(es, prop)
    }

    pub fn GetWindowHandle(&self) -> HWND {
        let lib = &nodeDll;
        let wkeGetWindowHandle: Symbol<GetWindowHandle> =
            unsafe { lib.get(b"wkeGetWindowHandle").unwrap() };
        wkeGetWindowHandle(self.webview)
    }

    pub fn FireWindowsMessage(
        &self,
        hWnd: HWND,
        message: i32,
        wParam: WPARAM,
        lParam: LPARAM,
    ) -> bool {
        let lib = &nodeDll;
        let wkeFireWindowsMessage: Symbol<FireWindowsMessage> =
            unsafe { lib.get(b"wkeFireWindowsMessage").unwrap() };

        wkeFireWindowsMessage(self.webview, hWnd, message, wParam, lParam)
    }
}
