use std::collections::HashMap;

#[derive(Clone)]
enum RispExpression {
    Symbol(String),
    Number(f64),
    List(Vec<RispExpression>)
}

#[derive(Debug)]
enum RispError {
    Reason(String)
}

#[derive(Clone)]
struct RispEnvironment {
    data: HashMap<String, RispExpression>
}

fn tokenize(expression: String) -> Vec<String> {
    expression
        .replace("(", " ( ")
        .replace(")", " ) ")
        .split_whitespace()
        .map(|x| x.to_string())
        .collect()
}

fn parse<'a>(token: &'a [String]) -> Result<(RispExpression, &'a [String]), RispError> {
    let (token, rest) = token.split_first()
        .ok_or(
            RispError::Reason("cloud not get token".to_string())
        )?;
}

fn main() {
    let exp = String::from("(+ 10 5)");

    println!("{:?}", tokenize(exp));
}

// https://stopa.io/post/222
