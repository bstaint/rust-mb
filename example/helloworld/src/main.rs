use miniblink::mb::*;
fn main() {
    MB::Initialize();
    MB::EnableHighDPISupport();

    let mut mb = MB::new();
    mb.CreateWebWindow(mbWindow::default())
        .LoadFile("./index.html")
        .MoveToCenter()
        .ShowWindow();

    MB::RunMessageLoop();
}
