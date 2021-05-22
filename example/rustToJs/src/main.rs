use miniblink::interface::Type::*;
fn main() {
    MB::Initialize();
    MB::EnableHighDPISupport();

    let mut mb = MB::new();
    mb.CreateWebWindow(Window::default())
        .MoveToCenter()
        .ShowWindow();

    mb.RunJS("alert('hello world')");
        
    MB::RunMessageLoop();
}





