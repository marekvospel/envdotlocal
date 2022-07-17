use anyhow::Result;

#[derive(Debug)]
pub struct Token {
  pub token: Tokens,
  pub start: usize,
  pub end: usize,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Tokens {
  Space,
  Export,
  EqSet,
  Literal(String),
  LineEnd(String),
}

impl Tokens {
  #[inline]
  fn detect<S: Into<String>>(str: S) -> Self {
    use Tokens::*;
    let str = str.into();
    match str.as_str() {
      " " => Space,
      "export" => Export,
      "=" => EqSet,
      "\n" | "\r\n" => LineEnd(str),
      _ => Literal(str),
    }
  }
}

impl ToString for Tokens {
  #[inline]
  fn to_string(&self) -> String {
    use Tokens::*;
    match self {
      Space => " ".into(),
      Export => "export".into(),
      EqSet => "=".into(),
      Literal(str) => str.into(),
      LineEnd(str) => str.into(),
    }
  }
}

pub fn tokenize(src: String) -> Result<Vec<Token>> {
  let mut tokens = Vec::new();

  fn save_token(buf: &mut String, tokens: &mut Vec<Token>, i: usize) {
    if !buf.is_empty() {
      tokens.push(Token {
        start: i - buf.len(),
        end: i,
        token: Tokens::detect(buf.to_owned()),
      });
      *buf = String::new();
    }
  }

  let mut buf = String::new();
  let mut quote_active = false;
  let mut double_quote_active = false;
  let mut triple_quote_active = false;
  let mut escape_active = false;
  let mut buf_add;
  let mut skip = 0;
  let mut double_quote_count = 0;

  for i in 0..src.len() {
    if skip > 0 {
      skip -= 1;
      continue;
    }
    buf_add = true;
    let c = src.chars().nth(i).unwrap();

    match c {
      ' ' => {
        if !escape_active && !quote_active && !double_quote_active && !triple_quote_active {
          save_token(&mut buf, &mut tokens, i);
          let mut x = i;
          while src.chars().nth(x) == Some(' ') {
            x += 1;
          }
          tokens.push(Token {
            token: Tokens::Space,
            start: i,
            end: x - 1,
          });
          skip += x - i - 1;
          buf_add = false;
        }
      }
      '\n' | '\r' => {
        if !escape_active && !triple_quote_active {
          save_token(&mut buf, &mut tokens, i);
          let mut x = i;
          while matches!(src.chars().nth(x), Some('\n') | Some('\r') | Some(' ')) {
            x += 1
          }
          tokens.push(Token {
            token: Tokens::LineEnd(src.chars().skip(i).take(x - i).collect()),
            start: i,
            end: x - 1,
          });
          skip += x - i - 1;
          buf_add = false;
        }
      }
      '#' => {
        if !escape_active && !quote_active && !double_quote_active && !triple_quote_active {
          save_token(&mut buf, &mut tokens, i);
          let mut x = i;
          while !matches!(src.chars().nth(x), Some('\n') | Some('\r')) {
            x += 1
          }
          while matches!(src.chars().nth(x), Some('\n') | Some('\r') | Some(' ')) {
            x += 1
          }
          skip += x - i - 1;
          buf_add = false
        }
      }
      '\\' => {
        if !quote_active {
          if !escape_active {
            escape_active = true;
            buf_add = false;
          } else {
            escape_active = false
          }
        }
      }
      '=' => {
        if !escape_active && !quote_active && !double_quote_active && !triple_quote_active {
          save_token(&mut buf, &mut tokens, i);
          tokens.push(Token {
            token: Tokens::EqSet,
            start: i,
            end: i,
          });
          buf_add = false;
        }
      }
      '\'' => {
        if !escape_active && !double_quote_active && !triple_quote_active {
          quote_active = !quote_active;
          buf_add = false;
        }
      }
      '"' => {
        if !escape_active && !quote_active {
          double_quote_count += 1;
        }

        if double_quote_count == 3 {
          if triple_quote_active {
            buf.drain((buf.len() - 2)..buf.len());
          } else {
            triple_quote_active = !triple_quote_active;
          }
          buf_add = false;
        }

        if !triple_quote_active {
          double_quote_active = !double_quote_active;
          buf_add = false;
        }
      }
      _ => {}
    }
    if c != '"' && !quote_active && !escape_active {
      double_quote_count = 0;
    }
    if c != '\\' && !quote_active {
      escape_active = false;
    }
    if buf_add {
      buf.push(c);
    }
  }
  save_token(&mut buf, &mut tokens, src.len());

  Ok(tokens)
}

#[cfg(test)]
mod tests {
  use crate::tokens::{tokenize, Tokens};

  #[test]
  fn tokens_should_be_detected() {
    assert_eq!(Tokens::Space, Tokens::detect(" "));
    assert_eq!(Tokens::Export, Tokens::detect("export"));
    assert_eq!(Tokens::EqSet, Tokens::detect("="));
    assert_eq!(Tokens::Literal("KEY".into()), Tokens::detect("KEY"));
    assert_eq!(Tokens::Literal("value".into()), Tokens::detect("value"));
    assert_eq!(Tokens::LineEnd("\n".into()), Tokens::detect("\n"));
    assert_eq!(Tokens::LineEnd("\r\n".into()), Tokens::detect("\r\n"));
  }

  #[test]
  fn tokens_should_be_converted_to_string() {
    assert_eq!(Tokens::Space.to_string(), " ".to_string());
    assert_eq!(Tokens::Export.to_string(), "export".to_string());
    assert_eq!(Tokens::Literal("let".into()).to_string(), "let".to_string());
    assert_eq!(Tokens::LineEnd("\n".into()).to_string(), "\n".to_string());
  }

  #[test]
  fn should_tokenize() {
    let input = r#"export HELLO=world #comment
    HI="no comment"
    SUS='usu'"#;
    println!("{:#?}", tokenize(input.into()));
  }
}
