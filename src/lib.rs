pub mod mb;

pub mod interface;

use interface::Type::*;

#[cfg(test)]
mod tests {

  use super::*;
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

    let mut mb = MB::new();
    mb.CreateWebWindow(window)
      .SetWindowTitle("窗口程序")
      .MoveToCenter()
      .loadUrl("http://127.0.0.1:8080/")
      .ShowWindow();

    // mb.RunJS("alert('hello world')");

    MB::RunMessageLoop();
  }

  fn getJSdata(es: jsExecState) -> jsValue {
    let jsArg = MB::jsArg(es, 0);
    let value = MB::jsToString(es, jsArg);

    println!("{}", value);
    let result = format!("你传入的值为{}，我是rust返回的值", value);
    return MB::jsString(es, &result);
  }
}
