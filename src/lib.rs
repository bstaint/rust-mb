#![allow(non_snake_case)]
#![allow(unused_imports)]

pub mod mb;

pub mod interface;

#[cfg(test)]
mod tests {

    use super::*;
    use lazy_static::lazy_static;
    use std::sync::Mutex;
    use interface::Type::*;

    lazy_static! {
        static ref mb: Mutex<MB> = Mutex::new(MB::new());
    }
    #[test]
    fn CreateWebWindow() {
        MB::Initialize();
        MB::EnableHighDPISupport();

        let window = Window {
            style: 0,
            parent: 0,
            x: 0,
            y: 0,
            width: 450,
            height: 350,
        };
        MB::JsBindFunction("sendData", getJSdata, 0);
        {
            let mut _mb = mb.lock().unwrap();
            _mb.CreateWebWindow(window)
                .SetWindowTitle("窗口")
                .loadUrl("http://127.0.0.1:8080/")
                .MoveToCenter()
                .ShowWindow();
        }

        // mb.RunJS("alert('hello world')");

        MB::RunMessageLoop();
    }

    fn getJSdata(es: jsExecState) -> jsValue {
        let jsArg = MB::jsArg(es, 0);
        let value = MB::jsToString(es, jsArg);

        let url = &mb.lock().unwrap().url;

        println!("{}", url);
        let result = format!("当前url为{}，我是rust返回的值", url);
        return MB::jsString(es, &result);
    }
}
