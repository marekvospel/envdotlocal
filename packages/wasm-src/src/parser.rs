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
