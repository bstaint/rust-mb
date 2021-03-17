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
        MB::JsBindFunction("sendData", getJSdata, 0);

        let window = Window {

            ..Default::default()
        };

        let mut mb = MB::new();
        mb.CreateWebWindow(window)
            .LoadFile("./index.html")
            .MoveToCenter()
            .ShowWindow();

        // mb.RunJS("alert('hello world')");

        MB::RunMessageLoop();
    }
   
    fn getJSdata(es: jsExecState) -> jsValue {
        let jsArg = MB::jsArg(es, 0);
       


        let object = MB::jsEmptyObject(es);
        MB::jsSet(es, object, "name", jsArg);
        object
    }
}
