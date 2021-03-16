#[macro_use]
extern crate neon;

extern crate url;

use url::{Url};
use neon::vm::{Call, JsResult};
use neon::js::{JsString, JsObject};

#[macro_use]//可以使用 info! 的log宏，打印log===>
extern crate log;



fn get_query(call: Call) -> JsResult<JsString> {
    let scope = call.scope;
    let url = call.arguments.require(scope, 0)?.check::<JsString>()?.value();

    let parsed_url = Url::parse(
        &url
    ).unwrap();

    Ok(JsString::new(scope, parsed_url.query().unwrap()).unwrap())
}

register_module!(ctx, {
    // 导出一个方法===>
    ctx.export("getQuery", get_query);
});