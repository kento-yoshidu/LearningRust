mod interpreter;

use interpreter::lexer;

fn main() {
    lexer::break_down_into_token("test");
}
