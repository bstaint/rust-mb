use miniblink::mb::*;
fn main() {
    MB::Initialize();
    MB::EnableHighDPISupport();

    let mut mb = MB::new();
    mb.CreateWebWindow(mbWindow::default())
        .MoveToCenter()
        .ShowWindow();
        

    mb.RunJS("alert('hello world')");
        
    MB::RunMessageLoop();
}





