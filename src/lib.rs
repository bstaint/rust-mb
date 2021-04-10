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
   

        let mut mb = MB::new();
        mb.CreateWebWindow(Window::default())
            .LoadFile("./index.html")
            .MoveToCenter()
            .ShowWindow();

        MB::RunMessageLoop();
    }

    fn getJSdata(es: jsExecState) -> jsValue {
        let jsArg = es.getArg(0);

        let jsobj = MB::jsEmptyObject(es);
        jsobj.setProp(es, "name", jsArg);

        jsobj
    }
}
