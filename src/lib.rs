pub mod mb;

pub mod interface;

use interface::Type::*;
use mb::MB;

#[cfg(test)]
mod tests {

  use super::*;
  #[test]
  fn CreateWebWindow() {
    let mut mb = MB::new();
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
    mb.CreateWebWindow(window);
    mb.SetWindowTitle("窗口程序");
    mb.MoveToCenter();
    mb.loadUrl("http://www.baidu.com/");
    mb.ShowWindow();

    MB::RunMessageLoop();
  }
}
