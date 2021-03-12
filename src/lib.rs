#![allow(non_snake_case)]
#![allow(unused_imports)]

pub mod mb;

pub mod interface;

#[cfg(test)]
mod tests {

    use super::*;
    use interface::Type::*;
    use lazy_static::lazy_static;
    use std::sync::Mutex;

    lazy_static! {
        static ref mb: Mutex<MB> = Mutex::new(MB::new());
    }
    #[test]
    fn CreateWebWindow() {
        MB::Initialize();
        MB::EnableHighDPISupport();

        let window = Window {
            width: 1330,
            height: 860,
            ..Default::default()
        };
        {
            let mut _mb = mb.lock().unwrap();
            _mb.CreateWebWindow(window)
                .SetWindowTitle("PostWoman")
                .loadUrl("http://127.0.0.1:8080/cn/")
                .MoveToCenter()
                .ShowWindow()
                .ShowDevtools(r#"file:///D:\documents\1152207863\FileRecv\miniblink-20210131 (4)\front_end\inspector.html"#);
        }

        // mb.RunJS("alert('hello world')");

        MB::RunMessageLoop();
    }

    fn getJSdata(es: jsExecState) -> jsValue {
        let jsArg = MB::jsArg(es, 0);
        let value = MB::jsToString(es, jsArg);

        let object = MB::jsEmptyObject(es);
        MB::jsSet(es, object, "name", jsArg);
        object
    }
}
