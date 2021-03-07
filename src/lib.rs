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

    let mut mb = MB::new()
      .CreateWebWindow(window)
      .SetWindowTitle("窗口程序")
      .MoveToCenter()
      .loadUrl("http://www.baidu.com/")
      .ShowWindow();

    MB::RunMessageLoop();
  }
}
