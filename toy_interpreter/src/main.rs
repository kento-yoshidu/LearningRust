#[derive(Debug)]
#[allow(unused)]
pub enum Token {
    PLUS,
    MINUS,
    Number(usize),
}

#[derive(Debug)]
struct Lexer {
    exp: Vec<char>,
}

impl Lexer {
    fn new(exp: Vec<char>) -> Lexer {
        Lexer {
            exp,
        }
    }

    fn tokenize(&self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();

        let exp = &self.exp;
        let mut pos = 0;
        let mut number_literal: Vec<char> = Vec::new();

        loop {
            if exp[pos] == ';' && pos == exp.len()-1 {

                let Ok(num) = parse_to_numeric(&number_literal) else {
                    eprintln!("数値文字列を数値に変換できませんでした。");
                    panic!();
                };

                tokens.push(Token::Number(num));
                break;
            }

            if exp[pos] == ';' {
                println!("不正な;を検出");
                panic!();
            }

            if exp[pos] == ' ' {
                pos += 1;
                continue;
            }

            if is_number(exp[pos]) {
                number_literal.push(exp[pos]);
            } else {
                let Ok(num) = parse_to_numeric(&number_literal) else {
                    eprintln!("数値文字列を数値に変換できませんでした。");
                    panic!();
                };

                tokens.push(Token::Number(num));

                match exp[pos] {
                    '+' => tokens.push(Token::PLUS),
                    '-' => tokens.push(Token::MINUS),
                    _ => (),
                }

                number_literal.clear();
            }

            pos += 1;
        }

        tokens
    }
}

#[allow(unused)]
fn main() {
    let l = Lexer::new("11 + 2;".chars().collect());

    println!("result = {:?}", l.tokenize());
}

// 数値かどうかの判定
fn is_number(c: char) -> bool {
    c.to_digit(10).is_some()
}

// String型の数値をusizeに変換
fn parse_to_numeric(s: &Vec<char>) -> Result<usize, String> {
    let str: String = s.iter().collect();

    let Ok(num) = str.parse::<usize>() else {
        panic!();
    };

    Ok(num)
}

#[cfg(test)]
mod tests {
    use super::{is_number, parse_to_numeric};

    #[test]
    fn test_is_number() {
        let inputs = [('1', true), ('2', true), ('a', false), (' ', false)];

        for (c, expected) in inputs {
            assert_eq!(is_number(c), expected);
        }
    }

    #[test]
    fn test_parse_to_numeric() {
        let inputs = [
            (vec!['1', '2', '3'], Ok(123)),
            (vec!['0', '1', '2'], Ok(12))];

        for (s, expected) in inputs {
            assert_eq!(parse_to_numeric(&s), expected);
        }
    }
}
