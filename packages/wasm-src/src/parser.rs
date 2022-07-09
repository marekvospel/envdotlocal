use pest::error::Error;
use pest::Parser;

#[derive(Parser)]
#[grammar = "./dotenv.pest"]
struct DotenvParser;

pub(crate) fn parse_dotenv(s: &str) -> Option<String> {
    match parse_inner(s) {
        Ok(v) => Some(v.join("\n")),
        Err(_) => None,
    }
}

fn parse_inner(s: &str) -> Result<Vec<String>, Error<Rule>> {
    let mut result = DotenvParser::parse(Rule::file, s)?;

    let file = result.next().unwrap();

    let lines = file.into_inner();

    let mut outstr = Vec::new();

    for line in lines.into_iter() {
        let mut line_inner = line.into_inner();

        let key_rule = line_inner.next();

        if let None = key_rule {
            continue;
        }

        let key: String = key_rule.unwrap().as_str().into();
        let mut value: String = "".into();

        let value_rule = line_inner.next();

        if let None = value_rule {
            outstr.push(format!("key: \"{}\", value: \"{}\"", key, value));
            continue;
        }

        value = value_rule
            .unwrap()
            .into_inner()
            .next()
            .unwrap()
            .as_str()
            .into();

        outstr.push(format!("key: \"{}\", value: \"{}\"", key, value));
    }

    Ok(outstr)
}

#[cfg(test)]
mod tests {
    use crate::parse_dotenv;

    #[test]
    fn should_parse_empty_value() {
        let result = parse_dotenv("EMPTY=");

        assert_eq!(
            result,
            Some("key: \"EMPTY\", value: \"\"".into())
        );
    }

    #[test]
    fn should_parse_unquoted() {
        let result = parse_dotenv("HELLO=hi");

        assert_eq!(
            result,
            Some("key: \"HELLO\", value: \"hi\"".into())
        );
    }

    #[test]
    fn should_parse_unquoted_ignore_after_space() {
        let result = parse_dotenv("HELLO=hi there");

        assert_eq!(
            result,
            Some("key: \"HELLO\", value: \"hi\"".into())
        );
    }

    #[test]
    fn should_parse_single_quote() {
        let result = parse_dotenv("HELLO='hi'");

        assert_eq!(
            result,
            Some("key: \"HELLO\", value: \"hi\"".into())
        );
    }

    #[test]
    fn should_parse_double_quote() {
        let result = parse_dotenv("HELLO=\"hi\"");

        assert_eq!(
            result,
            Some("key: \"HELLO\", value: \"hi\"".into())
        );
    }

    #[test]
    fn should_parse_triple_quote() {
        let result = parse_dotenv("HELLO=\"\"\"hi\"\"\"");

        assert_eq!(
            result,
            Some("key: \"HELLO\", value: \"hi\"".into())
        );
    }

    #[test]
    fn should_parse_triple_quote_multiline() {
        let result = parse_dotenv(r#"
HELLO="""hi
hello :)"""
"#
        );

        assert_eq!(
            result,
            Some("key: \"HELLO\", value: \"hi\nhello :)\"".into())
        );
    }

    #[test]
    fn should_parse_multiple() {
        let result = parse_dotenv(r#"
FIRST=1
SECOND='2'
THIRD="3"
FOURTH="""4"""
"#);

        assert_eq!(
            result,
            Some("key: \"FIRST\", value: \"1\"\nkey: \"SECOND\", value: \"2\"\nkey: \"THIRD\", value: \"3\"\nkey: \"FOURTH\", value: \"4\"".into())
        );
    }

    #[test]
    fn should_parse_indented() {
        let result = parse_dotenv("  HELLO='hi'");

        assert_eq!(
            result,
            Some("key: \"HELLO\", value: \"hi\"".into())
        );
    }
}
