use lexer::lex;


pub mod lexer;

fn main() {
    lex(&"%%%".to_string());
}
