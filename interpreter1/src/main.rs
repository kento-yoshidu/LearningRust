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

        match self.stack.pop() {
            Some(res2) => Some((res1, res2)),
            None => {
                println!("スタックにデータがありません。");
                self.stack.push(res1);
                None
            }

        }
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
    stack.push(1);
    stack.print();
    stack.push(2);
    stack.print();

    stack.pop();
    stack.print();

    stack.push(3);
    stack.print();

    stack.pop2();
    stack.print();

    stack.pop();
}

