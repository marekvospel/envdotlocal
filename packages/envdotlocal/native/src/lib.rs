use neon::prelude::*;
use crate::parser::parse_dotenv;

extern crate pest;
#[macro_use]
extern crate pest_derive;

pub(crate) mod parser;

fn parse(mut cx: FunctionContext) -> JsResult<JsObject> {
    let arg: Handle<JsString> = cx.argument(0)?;
    let arg = arg.value(&mut cx);
    let result = parse_dotenv(&arg);

    let out = cx.empty_object();

    for (key, value) in result.into_iter() {
        let value = cx.string(value);
        out.set(&mut cx, key.as_str(), value)?;
    }

    Ok(out)
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("parse", parse)?;
    Ok(())
}
