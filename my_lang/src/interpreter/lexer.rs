#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    // echo,
    // Number(f64),
    // Plus,
    LParen,
    RParen
}

#[derive(Debug)]
pub struct Lexer {
    // 入力された文字列
    string: Vec<char>,
    // 現在解析している位置
    position: usize,
}

impl Lexer {
    // 初期化
    pub fn new(string: Vec<char>) -> Self {
        Self {
            string,
            position: 0
        }
    }

    // 文字列をトークンに分解する
    pub fn token(&mut self) -> Option<Token> {
        let current = self.current();

        let token = match current {
            Some(&'(') => Some(Token::LParen),
            Some(&')') => Some(Token::RParen),
            _ => None
        };

        self.next();
        return token;
    }

    // 現在解析中の文字
    fn current(&mut self) -> Option<&char> {
        self.string.get(self.position)
    }

    // 位置を一つ次に進める
    fn next(&mut self) {
        self.position += 1;
    }
}

pub fn break_down_into_token(arg: &str) /*-> Option<Token>*/ {
    let mut lexer = Lexer::new(arg.chars().collect());

    println!("{:?}", lexer.token());
    println!("{:?}", lexer.token());
    println!("{:?}", lexer.token());
    println!("{:?}", lexer.token());
    println!("{:?}", lexer.token());
    println!("{:?}", lexer.token());
    println!("{:?}", lexer.token());
    println!("{:?}", lexer.token());
    println!("{:?}", lexer.token());
    println!("{:?}", lexer.token());
    println!("{:?}", lexer.token());
    println!("{:?}", lexer.token());
    println!("{:?}", lexer.token());

    // lexer
}

// 文字列をトークンに分解する
/*
pub fn break_down_into_token(arg: &str) -> Vec<&str> {
    let reg = Regex::new(r"print|(|)").unwrap();

    let vec: Vec<&str> = reg.split(arg).collect();

    vec
}
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Some(Token::LParen), break_down_into_token("("));
        assert_eq!(Some(Token::RParen), break_down_into_token(")"));
        assert_eq!(None, break_down_into_token("Hello World"));
    }
}
