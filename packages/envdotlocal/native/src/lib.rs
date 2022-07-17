use dotenv_parser::ast::{create_tree, Expression, Value as DotenvValue};
use dotenv_parser::tokens::tokenize;
use neon::prelude::*;

fn parse(mut cx: FunctionContext) -> JsResult<JsObject> {
  let arg: Handle<JsString> = cx.argument(0)?;
  let arg = arg.value(&mut cx);

  let tokens = tokenize(arg).unwrap();
  let result = create_tree(tokens);

  let out = cx.empty_object();

  for tree_node in result.into_iter() {
    let Expression::SetExpression(e) = tree_node;
    let value = cx.string(match e.value {
      DotenvValue::Literal(s) => s,
    });
    out.set(
      &mut cx,
      match e.key {
        DotenvValue::Literal(s) => s,
      }
      .as_str(),
      value,
    )?;
  }

  Ok(out)
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
  cx.export_function("parse", parse)?;
  Ok(())
}
