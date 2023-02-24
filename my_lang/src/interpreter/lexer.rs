use regex::Regex;

// 文字列をトークンに分解する
pub fn break_down_into_token(arg: &str) -> Vec<&str> {
    let reg = Regex::new(r",").unwrap();

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
