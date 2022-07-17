use crate::tokens::{Token, Tokens};

#[derive(Debug)]
pub struct SetExpression {
  pub key: Value,
  pub value: Value,
  pub export: bool,
}

#[derive(Debug)]
pub enum Expression {
  SetExpression(SetExpression),
}

#[derive(Debug)]
pub enum Value {
  Literal(String),
}

#[derive(Debug)]
struct Tree {
  tokens: Vec<Token>,
  i: usize,
}

impl Tree {
  fn parse_set(&mut self) -> Option<Expression> {
    if self.tokens.len() < self.i + 2 {
      return None;
    }
    let mut export = false;

    if matches!(self.get_current_token(), Tokens::Export) {
      export = true;
      self.inc();
      self.inc();

      if self.tokens.len() < self.i + 2 {
        return None;
      }
    }

    let key;
    let value;

    if let Tokens::Literal(str) = self.get_current_token() {
      key = str.clone();
    } else {
      return None;
    }
    self.inc();

    if !matches!(self.get_current_token(), Tokens::EqSet) {
      return None;
    }
    self.inc();

    if let Tokens::Literal(str) = self.get_current_token() {
      value = str.clone();
    } else {
      return None;
    }
    self.inc();

    Some(Expression::SetExpression(SetExpression {
      export,
      key: Value::Literal(key),
      value: Value::Literal(value),
    }))
  }

  fn get_expression(&mut self) -> Option<Expression> {
    let mut token = self.get_current_token();

    loop {
      match token {
        Tokens::Space => {
          self.inc();
        }
        Tokens::LineEnd(_) => {
          self.inc();
        }
        Tokens::Export => return self.parse_set(),
        Tokens::Literal(_) => return self.parse_set(),
        _ => {
          self.inc();
        }
      }
      if self.i >= self.tokens.len() - 1 {
        break;
      }
      token = self.get_current_token();
    }

    None
  }

  fn inc(&mut self) -> &Self {
    self.i += 1;
    self
  }
  fn get_current_token(&self) -> &Tokens {
    &self.tokens.get(self.i).unwrap().token
  }
}

pub fn create_tree(tokens: Vec<Token>) -> Vec<Expression> {
  let mut expressions = Vec::new();
  let mut tree = Tree { tokens, i: 0 };

  loop {
    if tree.i >= tree.tokens.len() - 1 {
      break;
    }
    let expression = tree.get_expression();
    if let Some(ex) = expression {
      expressions.push(ex);
    }
  }

  expressions
}

#[cfg(test)]
mod tests {
  use crate::ast::create_tree;
  use crate::tokens::tokenize;
  use anyhow::Result;

  #[test]
  fn should_create_ast() -> Result<()> {
    let input = r#"export HELLO=world #comment
    HI="no comment"
    SUS='usu'"#;
    let tokens = tokenize(input.into())?;
    let _ast = create_tree(tokens);

    Ok(())
  }
}
