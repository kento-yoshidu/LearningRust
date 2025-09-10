use std::io::{self, Write};

trait StackTrait<T> {
    fn push(&mut self, v: T);
    fn pop(&mut self) -> Option<T>;
    fn pop2(&mut self) -> Option<(T, T)>;
    fn print(&self);
}

struct Stack<T> {
    stack: Vec<T>,
}

impl<T: std::fmt::Debug> StackTrait<T> for Stack<T> {
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

fn main() {
    let mut stack = Stack::new();

    loop {
        print!("-> ");
        std::io::stdout().flush().unwrap();

        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
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

        match line {
            "+" => {
                if let Some((a, b)) = stack.pop2() {
                    let c = a + b;
                    stack.push(c);
                    println!("{c}");
                }
            },
            "-" => {
                if let Some((a, b)) = stack.pop2() {
                    let c = a - b;
                    stack.push(c);
                    println!("{}", c);
                }
            }
            "*" => {
                if let Some((a, b)) = stack.pop2() {
                    let c = a * b;
                    stack.push(c);
                    println!("{}", c);
                }
            }
            "/" => {
                if let Some((a, b)) = stack.pop2() {
                    let c = a / b;
                    stack.push(c);
                    println!("{}", c);
                }
            },
            _ => {
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
}
