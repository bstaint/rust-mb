#![allow(non_snake_case)]
#![allow(unused_imports)]

pub mod mb;

pub mod interface;

#[cfg(test)]
mod tests {


    use super::*;

    use interface::Type::*;


   

    #[test]
    fn CreateWebWindow() {
        MB::Initialize();
        MB::EnableHighDPISupport();

 
        let mut mb = MB::new();
        mb.CreateWebWindow(Window::default())
            .LoadFile("./www/index.html")
            .MoveToCenter()
            .ShowWindow()
            .ShowDevtools("file:///devtools/inspector.html");

        MB::RunMessageLoop();
    }}