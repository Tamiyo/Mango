use crate::core::LexerResult;
use crate::lexer::Lexer;

mod lexer;
mod core;

fn main() {
    let my_string = "0101.100 myident \"mystring\" 24012 >= ! == === >> %== =%= <= <> <===";
    let lexer = Lexer { input: my_string };
    lexer.get_tokens();
}
