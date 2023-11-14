fn main() {
    let input = "{()}";

    let mut output: String = String::default();
    for c in input.chars().into_iter() {
        output.push_str(
            match c {
                ')' => Token::RParen,
                '{' => Token::LBrace,
                '(' => Token::LParen,
                '}' => Token::RBrace,
                _ => Token::Nil,
            }
            .as_str(),
        );
    }

    println!("{}",output)
}

enum Token {
    LParen,
    RParen,
    LBrace,
    RBrace,

    Nil,
}

impl Token {
    fn as_str(&self) -> &str {
        match self {
            Token::LParen => "(",
            Token::RParen => ")",
            Token::LBrace => "{",
            Token::RBrace => "}",
            Token::Nil => "nil",
        }
    }
}
