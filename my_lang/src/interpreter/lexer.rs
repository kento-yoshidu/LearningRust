use regex::Regex;

#[derive(Debug, PartialEq, Clone)]
enum Token {
    echo,
    Number(f64),
    Plus,
    LParen,
    RParen
}

#[derive(Debug)]
struct Lexer {
    string: Vec<char>,
    position: usize,
}

impl Lexer {
    // 初期化
    fn new(string: Vec<char>) -> Self {
        Self {
            string,
            position: 0
        }
    }

    // 現在解析中の文字
    fn current(&mut self) -> Option<&char> {
        self.string.get(self.position)
    }

    /*
    fn token(&mut self) -> Option<Token> {
        let current = self.current()?;
    }
    */
}

pub fn test() {
    let mut lexer = Lexer::new("Hello World".chars().collect());

    let current = lexer.current();

    println!("{:?}", current);
}

// 文字列をトークンに分解する
pub fn break_down_into_token(arg: &str) -> Vec<&str> {
    let reg = Regex::new(r"print|(|)").unwrap();

    let vec: Vec<&str> = reg.split(arg).collect();

    vec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let a1 = vec!["Hello", "World"];
        assert_eq!(a1, break_down_into_token("Hello,World"));
    }
}
