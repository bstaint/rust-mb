#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
use super::util::*;

use crate::interface::Func::*;
use crate::interface::Type::*;
use lazy_static::lazy_static;
use libloading::{Library, Symbol};

lazy_static! {
    static ref nodeDll: Library =
        unsafe { Library::new("./node.dll").expect("引用node.dll出错！") };
}

impl MB {
    pub fn new() -> MB {
        MB {
            webview: 0,
            url: String::new(),
        }
    }
    /**初始化miniblink */
    pub fn Initialize() {
        let lib = &nodeDll;
        let wkeInitialize: Symbol<Initialize> = unsafe { lib.get(b"wkeInitialize").unwrap() };

        wkeInitialize();
    }

    /**创建一个窗口 */
    pub fn CreateWebWindow(&mut self, window: Window) -> &mut MB {
        let lib = &nodeDll;
        let wkeCreateWebWindow: Symbol<CreateWebWindow> =
            unsafe { lib.get(b"wkeCreateWebWindow").unwrap() };
        let webview = wkeCreateWebWindow(
            window.style,
            window.parent,
            window.x,
            window.y,
            window.width,
            window.height,
        );
        self.webview = webview;
        self
    }

    /**显示窗口 */
    pub fn ShowWindow(&mut self) -> &mut MB {
        let lib = &nodeDll;
        let wkeShowWindow: Symbol<ShowWindow> = unsafe { lib.get(b"wkeShowWindow").unwrap() };

        wkeShowWindow(self.webview, true);
        self
    }

    /**窗口居中 */
    pub fn MoveToCenter(&mut self) -> &mut MB {
        let lib = &nodeDll;
        let wkeMoveToCenter: Symbol<MoveToCenter> = unsafe { lib.get(b"wkeMoveToCenter").unwrap() };
        wkeMoveToCenter(self.webview);
        self
    }

    /**开启高dpi支持 */
    pub fn EnableHighDPISupport() {
        let lib = &nodeDll;
        let wkeEnableHighDPISupport: Symbol<EnableHighDPISupport> =
            unsafe { lib.get(b"wkeEnableHighDPISupport").unwrap() };
        wkeEnableHighDPISupport();
    }

    /**开启消息循环 */
    pub fn RunMessageLoop() {
        let lib = &nodeDll;
        let wkeRunMessageLoop: Symbol<RunMessageLoop> =
            unsafe { lib.get(b"wkeRunMessageLoop").unwrap() };
        wkeRunMessageLoop();
    }

    /**设置窗口标题 */
    pub fn SetWindowTitle(&mut self, title: &str) -> &mut MB {
        let lib = &nodeDll;
        let wkeSetWindowTitle: Symbol<SetWindowTitle> =
            unsafe { lib.get(b"wkeSetWindowTitle").unwrap() };
        let c_title = rustToCStr(title);
        wkeSetWindowTitle(self.webview, c_title);
        self
    }

    /**加载URL */
    pub fn loadUrl(&mut self, url: &str) -> &mut MB {
        let lib = &nodeDll;
        let wkeLoadURL: Symbol<LoadUrl> = unsafe { lib.get(b"wkeLoadURL").unwrap() };
        self.url = String::from(url);

        let c_url = rustToCStr(url);
        wkeLoadURL(self.webview, c_url);
        self
    }

    /**窗体销毁回调 */
    pub fn OnWindowDestroy(&mut self, callback: fn()) -> &mut MB {
        let lib = &nodeDll;
        let wkeOnWindowDestroy: Symbol<OnWindowDestroy> =
            unsafe { lib.get(b"wkeOnWindowDestroy").unwrap() };

        wkeOnWindowDestroy(self.webview, callback);
        self
    }

    /**JS绑定方法 */
    pub fn JsBindFunction(msg: &str, callback: fn(es: jsExecState) -> jsValue, args: i32) {
        let lib = &nodeDll;
        let wkeJsBindFunction: Symbol<JsBindFunction> =
            unsafe { lib.get(b"wkeJsBindFunction").unwrap() };
        let c_msg = rustToCStr(msg);
        wkeJsBindFunction(c_msg, callback, args);
    }

    /**执行JS */
    pub fn RunJS(&mut self, script: &str) -> jsValue {
        let lib = &nodeDll;
        let wkeRunJS: Symbol<RunJS> = unsafe { lib.get(b"wkeRunJS").unwrap() };

        let script = rustToCStr(script);
        wkeRunJS(self.webview, script)
    }
    /** 打开开发者工具 */
    pub fn ShowDevtools(&mut self, path: &str) {
        let lib = &nodeDll;
        let ShowDevtools: Symbol<ShowDevtools> = unsafe { lib.get(b"wkeShowDevtools").unwrap() };
        let path = rustToCStrW(path);
        ShowDevtools(self.webview, path as *mut u16, 0, 0);
    }

    /** 获取网页HTML */
    pub fn GetSource<'a>(&mut self) -> &'a str {
        let lib = &nodeDll;
        let wkeGetSource: Symbol<GetSource> = unsafe { lib.get(b"wkeGetSource").unwrap() };

        let source = wkeGetSource(self.webview);
        cToRustStr(source)
    }

    /** 获取当前线程 Webview */
    pub fn GetWebViewForCurrentContext<'a>() -> Webview {
        let lib = &nodeDll;
        let GetWebViewForCurrentContext: Symbol<GetWebViewForCurrentContext> =
            unsafe { lib.get(b"wkeGetWebViewForCurrentContext").unwrap() };

        GetWebViewForCurrentContext()
    }

    /** 获取当前的url */
    pub fn GetURL<'a>(&mut self) -> &'a str {
        let lib = &nodeDll;
        let wkeGetURL: Symbol<GetURL> = unsafe { lib.get(b"wkeGetURL").unwrap() };

        let url = wkeGetURL(self.webview);
        cToRustStr(url)
    }

    /** 获取当前的MB */
    pub fn GetCurrentMB() -> MB {
        let webview = MB::GetWebViewForCurrentContext();
        MB {
            webview: webview,
            url: String::new(),
        }
    }

    /**获取传入参数 */
    pub fn jsArg(es: jsExecState, agrId: i32) -> jsValue {
        let lib = &nodeDll;
        let jsArg: Symbol<jsArg> = unsafe { lib.get(b"jsArg").unwrap() };

        jsArg(es, agrId)
    }

    /**转换成String  */
    pub fn jsToString<'a>(es: jsExecState, value: jsValue) -> &'a str {
        let lib = &nodeDll;
        let jsToString: Symbol<jsToString> = unsafe { lib.get(b"jsToString").unwrap() };

        let msg = jsToString(es, value);

        cToRustStr(msg)
    }

    pub fn jsString(es: jsExecState, str: &str) -> jsValue {
        let lib = &nodeDll;
        let jsString: Symbol<jsString> = unsafe { lib.get(b"jsString").unwrap() };

        let str = rustToCStr(str);
        jsString(es, str)
    }

    pub fn jsEmptyObject(es: jsExecState) -> jsValue {
        let lib = &nodeDll;
        let jsEmptyObject: Symbol<jsEmptyObject> = unsafe { lib.get(b"jsEmptyObject").unwrap() };

        jsEmptyObject(es)
    }

    pub fn jsSet(es: jsExecState, object: jsValue, prop: &str, value: jsValue) {
        let lib = &nodeDll;
        let jsSet: Symbol<jsSet> = unsafe { lib.get(b"jsSet").unwrap() };
        let msg = rustToCStr(prop);
        jsSet(es, object, msg, value);
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
