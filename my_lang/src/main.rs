mod interpreter;

use interpreter::lexer;

fn main() {
    println!("{:?}", lexer::test());
}
