#![allow(non_camel_case_types)]
#[warn(unused_parens)]
use crate::interface::Type::*;

impl jsValue {
    ///转换成字符串
    pub fn toString<'a>(&self, es: jsExecState) -> &'a str {
        MB::jsToString(es, self.clone())
    }
    
    ///转换成i32
    pub fn toInt(&self, es: jsExecState) -> i32 {
        MB::jsToInt(es, self.clone())
    }

    ///转换成f32
    pub fn toFloat(&self, es: jsExecState) -> f32 {
        MB::jsToFloat(es, self.clone())
    }

    ///获取对象属性
    pub fn getProp(&self, es: jsExecState, prop: &str) -> jsValue {
        MB::jsGet(es, self.clone(), prop)
    }

    ///获取对象属性
    pub fn setProp(&self, es: jsExecState, prop: &str, value: jsValue) {
        MB::jsSet(es, self.clone(), prop, value);
    }
}
