extern crate neon;

#[macro_use]//可以使用 info! 的log宏，打印log===>
extern crate log;

mod sync;
use sync::JsFileWatcher;

use neon::prelude::*;

fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string("hello neon"))
}

// 注册导出 & binding
register_module!(mut cx, {
    // 导出一个方法
    cx.export_function("hello", hello);
    // 导出一个class
    cx.export_class::<JsFileWatcher>("NodeFileWatcher")
});
