use miniblink::interface::Type::*;
fn main() {
    MB::Initialize();
    MB::EnableHighDPISupport();
    MB::JsBindFunction("sendStr",sendStr,0);
    MB::JsBindFunction("sendObj",sendObj,0);

    let mut mb = MB::new();
    mb.CreateWebWindow(Window::default())
        .LoadFile("./index.html")
        .MoveToCenter()
        .ShowWindow();
        
    MB::RunMessageLoop();
}

fn sendStr(es: jsExecState)->jsValue{
    
    let arg=es.getArg(0);
    let str=arg.toString(es);
    
    println!("{:?}",str);

    MB::jsUndefined()
}

fn sendObj(es: jsExecState)->jsValue{
    
    let arg=es.getArg(0);
    let name=arg.getProp(es,"name").toString(es);
    let age=arg.getProp(es,"age").toString(es);
    println!("name:{:?},age:{:?}",name,age);

    MB::jsUndefined()
}




