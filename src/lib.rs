#![allow(non_snake_case)]
#![allow(unused_imports)]

pub mod mb;

mod interface;

#[path = "./mb/MB.rs"]
mod MB;
#[path = "./mb/Netjob.rs"]
mod Netjob;
#[path = "./mb/jsExecState.rs"]
mod jsExecState;
#[path = "./mb/jsValue.rs"]
mod jsValue;

#[cfg(test)]
mod tests {

    use std::process::exit;

    use crate::mb::*;

    #[test]
    fn CreateWebWindow() {
        MB::Initialize();
        MB::EnableHighDPISupport();

        MB::JsBindFunction(
            "test",
            |es| {
                let title = MB::GetCurrentMB().GetTitle();

                MB::jsString(es, title)
            },
            0,
        );

        let mut mb = MB::new();
        mb.CreateWebWindow(mbWindow::default())
            .LoadFile("./www/index.html")
            .MoveToCenter()
            .ShowWindow()
            .OnWindowDestroy(OnWindowDestroy)
            .ShowDevtools("file:///devtools/inspector.html");

        MB::RunMessageLoop();
    }

    fn OnWindowDestroy() {
        exit(0);
    }
}
