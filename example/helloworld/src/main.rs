use miniblink::interface::Type::*;
fn main() {
    MB::Initialize();
    MB::EnableHighDPISupport();

    let mut mb = MB::new();
    mb.CreateWebWindow(Window::default())
        .LoadFile("./index.html")
        .MoveToCenter()
        .ShowWindow();

    MB::RunMessageLoop();
}
