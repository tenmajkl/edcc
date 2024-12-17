use lexer::lex;


pub mod lexer;

fn main() {
    println!("{:?}", lex(&"11+1\n's'+'x'".to_string()));
}
