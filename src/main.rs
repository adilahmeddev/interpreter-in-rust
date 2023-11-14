fn main() {
    let input = "{()}";
    let mut iter = input.chars().enumerate().peekable();
    let mut output: String = String::default();
    
    while let Some((i, c)) = iter.next(){
        output.push_str(
            match c {
                ')' => Token::RParen,
                '{' => Token::LBrace,
                '(' => Token::LParen,
                '}' => Token::RBrace,
                '+' => Token::Plus,
                '-' => Token::Minus,
                '!' =>   match iter.peek() == Some(&(i+1,'=')) {
                                true => Token::NotEqual,
                                false => Token::Bang,
                        },
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
    
    Bang,
    Plus,
    Minus,
    Assign,
    Equal,
    NotEqual,
    Asterisk,
    BSlash,

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
            Token::Bang => "!",
            Token::Plus => "+",
            Token::Minus => "-",
            Token::Assign => "=",
            Token::Equal => "==",
            Token::NotEqual => "!=",
            Token::Asterisk => "*",
            Token::BSlash => "\\",
        }
    }
}
