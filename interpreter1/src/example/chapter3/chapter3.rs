use std::io::{Write, stdout, stdin};
use std::ops::{Add, Sub, Mul, Div};
use std::fmt::Debug;

trait StackTrait<T> {
    fn push(&mut self, v: T);
    fn pop(&mut self) -> Option<T>;
    fn pop2(&mut self) -> Option<(T, T)>;
    fn print(&self);
}

struct Stack<T> {
    stack: Vec<T>,
}

impl<T:
    Add<Output=T> +
    Sub<Output=T> +
    Mul<Output=T> +
    Div<Output=T> +
    Copy +
    Debug
> StackTrait<T> for Stack<T>
{
    fn push(&mut self, v: T) {
        self.stack.push(v);
    }

    fn pop(&mut self) -> Option<T> {
        match self.stack.pop() {
            Some(result) => Some(result),
            None => {
                println!("スタックにデータがありません。");
                None
            }
        }
    }

    fn pop2(&mut self) -> Option<(T, T)> {
        let res1 = match self.stack.pop() {
            Some(v) => v,
            None => {
                println!("スタックにデータがありません。");
                return None;
            }

        };

        let res2 = match self.stack.pop() {
            Some(v) => v,
            None => {
                println!("スタックにデータがありません。");
                self.stack.push(res1);
                return None;
            }

        };

        Some((res2, res1))
    }

    fn print(&self) {
        println!("{:?}", self.stack);
    }
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack { stack: Vec::new() }
    }
}

fn operate<T>(op: &str, stack: &mut Stack<T>)
where
    T: Add<Output=T> +
    Sub<Output=T> +
    Mul<Output=T> +
    Div<Output=T> +
    Copy +
    Debug,
{
    match op {
        "+" => {
            if let Some((a, b)) = stack.pop2() {
                let c = a + b;
                stack.push(c);
                println!("{:?}", c);
            }
        },
        "-" => {
            if let Some((a, b)) = stack.pop2() {
                let c = a - b;
                stack.push(c);
                println!("{:?}", c);
            }
        }
        "*" => {
            if let Some((a, b)) = stack.pop2() {
                let c = a * b;
                stack.push(c);
                println!("{:?}", c);
            }
        }
        "/" => {
            if let Some((a, b)) = stack.pop2() {
                let c = a / b;
                stack.push(c);
                println!("{:?}", c);
            }
        },
        _ => unreachable!()
    }
}

fn main() {
    let mut stack = Stack::new();

    loop {
        print!("-> ");
        stdout().flush().unwrap();

        let mut line = String::new();
        stdin().read_line(&mut line).unwrap();
        let line = line.trim();

        if line == "printstack" {
            stack.print();
            continue;
        }

        if line == "quit" || line == "exit" {
            break;
        }

        if line.to_lowercase() == "exit" {
            println!("もしかしてexit");
        }
        if line.to_lowercase() == "quit" {
            println!("もしかしてquit");
        }

        if line == "+" || line == "-" || line == "*" || line == "/" {
            operate(line, &mut stack);
        } else {
            match line.parse::<f64>() {
                Ok(v) => {
                    stack.push(v);
                    println!("{v}");
                },
                Err(_) => {
                    println!("シンタックスエラー");
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

   #[test]
    fn test_pop2_ok() {
        let mut stack = Stack::new();
        stack.push(1.0);
        stack.push(2.0);

        let res = stack.pop2();
        assert_eq!(res, Some((1.0, 2.0)));
    }

    #[test]
    fn test_pop2_not_enough_elements() {
        let mut stack = Stack::new();
        stack.push(42.0);

        let res = stack.pop2();
        assert_eq!(res, None);
        assert_eq!(stack.pop(), Some(42.0));
    }

    #[test]
    fn test_operate_add() {
        let mut stack = Stack::new();
        stack.push(2.0);
        stack.push(3.0);

        operate("+", &mut stack);

        assert_eq!(stack.pop(), Some(5.0));
    }

    #[test]
    fn test_operate_mul() {
        let mut stack = Stack::new();
        stack.push(4.0);
        stack.push(5.0);

        operate("*", &mut stack);

        assert_eq!(stack.pop(), Some(20.0));
    }
}
