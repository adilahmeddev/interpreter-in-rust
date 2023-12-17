use crate::lexer::Lexer;

pub mod token;
pub mod lexer;
pub mod parser;

fn main() {
    let input = String::from("{()}/*/* != == =         adil let le");
    
    let lexer = Lexer::new(input); 
    
    println!("{}",lexer.to_string())
}

