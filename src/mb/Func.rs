use winapi::{
    shared::{
        minwindef::{HINSTANCE, LPARAM, WPARAM},
        ntdef::LPCSTR,
        windef::HWND,
    },
    um::{
        libloaderapi::GetModuleHandleA,
        winuser::{LoadIconA, PostMessageA, ICON_SMALL, WM_SETICON},
    },
};

use crate::interface::Type::MB;

impl MB {
    /** 获取当前的MB */
    pub fn GetCurrentMB() -> MB {
        let webview = MB::GetWebViewForCurrentContext();
        MB {
            webview: webview,
            url: String::new(),
        }
    }

    /**设置窗口图标 */
    pub fn SetWindowIcon(&mut self, iconId: i32) -> &mut MB {
        let hwnd = self.GetWindowHandle();

        unsafe {
            let handle = GetModuleHandleA(0 as LPCSTR);
            let hIcon = LoadIconA(handle as HINSTANCE, iconId as LPCSTR);
            PostMessageA(
                hwnd as HWND,
                WM_SETICON,
                ICON_SMALL as WPARAM,
                hIcon as LPARAM,
            );
        };

        self
    }
}
