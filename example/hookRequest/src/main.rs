use miniblink::{interface::Type::*, mb::util::cToRustStr};
fn main() {
    MB::Initialize();
    MB::EnableHighDPISupport();

    let mut mb = MB::new();
    mb.CreateWebWindow(Window::default())
        .LoadFile("./index.html")
        .OnLoadUrlBegin(OnLoadUrlBeginCallBack, 0)
        .OnLoadUrlEnd(OnLoadUrlEndCallBack, 0)
        .MoveToCenter()
        .ShowWindow();

    MB::RunMessageLoop();
}

fn OnLoadUrlBeginCallBack(webView: Webview, params: i32, url: *const i8, job: Netjob) {
    job.HookRequest();
}

fn OnLoadUrlEndCallBack(
    webView: Webview,
    params: i32,
    url: *const i8,
    job: Netjob,
    buf: *const i8,
    len: i32,
) {
    let res = cToRustStr(buf);

    let mut mb = MB::GetMbByWebview(webView);
    mb.RunJS(&format!("result.value=`{}`", res));
}
