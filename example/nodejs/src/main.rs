use miniblink::mb::*;
///Nodejs只支持32位
fn main() {
    MB::Initialize();
    MB::EnableHighDPISupport();


    let mut mb = MB::new();
    mb.CreateWebWindow(mbWindow::default())
        .SetDebugConfig("enableNodejs", "1")
        .LoadFile("./index.html")
        .MoveToCenter()
        .ShowWindow();

    MB::RunMessageLoop();
}
