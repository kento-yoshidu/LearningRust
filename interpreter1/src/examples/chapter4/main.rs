use std::io::{stdin, stdout, Write};
use std::fmt::Debug;

#[derive(Debug)]
struct Stack {
    data: Vec<String>,
}

impl Stack {
    fn new() -> Self {
        Self { data: Vec::new() }
    }
    fn push(&mut self, v: String) {
        self.data.push(v);
    }
    fn pop(&mut self) -> Option<String> {
        self.data.pop()
    }
    fn len(&self) -> usize {
        self.data.len()
    }
}

fn operate(op: &str, stack: &mut Stack, v1: f64, v2: f64) {
    let res = match op {
        "+" => v1 + v2,
        "-" => v1 - v2,
        "*" => v1 * v2,
        "/" => v1 / v2,
        _ => unreachable!()
    };

    println!("{res}");
    stack.push(res.to_string());
}

fn is_op(s: &str) -> bool {
    s == "+" || s == "-" || s == "*" || s == "/"
}

fn main() {
    let mut stack: Stack = Stack::new();

    loop {
        print!("-> ");
        stdout().flush().unwrap();

        let mut line = String::new();
        stdin().read_line(&mut line).unwrap();
        let line = line.trim();

        if line == "debug" {
            println!("{:?}, len={}", stack, stack.len());
            continue;
        }

        if is_op(line) {
            if stack.len() < 1 {
                eprintln!("シンタックスエラー: 数値が足りません");
                continue;
            }

            let v = stack.pop().unwrap();

            if is_op(&v) {
                eprintln!("シンタックスエラー2");
            } else {
                stack.push(v);
                stack.push(line.to_string());
            }
        } else {
            let Ok(v) = line.parse::<f64>() else {
                eprintln!("数値に変換できませんでした");
                continue;
            };

            if stack.len() == 0 {
                stack.push(v.to_string());
                continue;
            }

            if stack.len() == 1 {
                eprint!("シンタックスエラー");
                continue;
            }

            let op = stack.pop().unwrap();
            let v2 = stack.pop().unwrap();

            operate(op.as_str(), &mut stack, v2.parse().unwrap(), v);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stack_push_pop() {
        let mut stack = Stack::new();
        stack.push("1".to_string());
        stack.push("2".to_string());

        assert_eq!(stack.len(), 2);
        assert_eq!(stack.pop(), Some("2".to_string()));
        assert_eq!(stack.pop(), Some("1".to_string()));
        assert_eq!(stack.pop(), None);
        assert_eq!(stack.len(), 0);
    }

    #[test]
    fn test_operate_add() {
        let mut stack = Stack::new();
        operate("+", &mut stack, 3.0, 4.0);
        assert_eq!(stack.pop().unwrap(), "7".to_string());
    }

    #[test]
    fn test_operate_sub() {
        let mut stack = Stack::new();
        operate("-", &mut stack, 10.0, 4.0);
        assert_eq!(stack.pop().unwrap(), "6".to_string());
    }

    #[test]
    fn test_operate_mul() {
        let mut stack = Stack::new();
        operate("*", &mut stack, 3.0, 5.0);
        assert_eq!(stack.pop().unwrap(), "15".to_string());
    }

    #[test]
    fn test_operate_div() {
        let mut stack = Stack::new();
        operate("/", &mut stack, 20.0, 5.0);
        assert_eq!(stack.pop().unwrap(), "4".to_string());
    }

    #[test]
    fn test_is_op() {
        assert!(is_op("+"));
        assert!(is_op("-"));
        assert!(is_op("*"));
        assert!(is_op("/"));
        assert!(!is_op("a"));
        assert!(!is_op("1"));
    }
}
