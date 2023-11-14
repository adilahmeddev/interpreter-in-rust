fn main() {
    let input = "{()}/*/* != == =";
    let mut iter = input.chars().enumerate().peekable();
    let mut output: String = String::default();

    while let Some((i, c)) = iter.next() {
        if c == ' ' { 
            continue;
        }
        output.push_str(
            match c {
                ')' => Token::RParen,
                '{' => Token::LBrace,
                '(' => Token::LParen,
                '}' => Token::RBrace,
                '+' => Token::Plus,
                '-' => Token::Minus,
                '!' => match iter.peek() == Some(&(i + 1, '=')) {
                    true => {
                        iter.next();
                        Token::NotEqual
                    },
                    false => Token::Bang,
                },
                '=' => match iter.peek() == Some(&(i + 1, '=')) {
                    true => {
                        iter.next();
                        Token::Equal
                    },
                    false => Token::Assign,
                },
                '/' => Token::Slash,
                '*' => Token::Asterisk,
                'l' => match iter.peek() == Some(&(i + 1, 'e')) {
                    true => {
                        iter.next();
                        Token::Equal
                    },
                    false => Token::Assign,
                }
               _ => Token::Nil,
            }
            .as_str(),
        );
    }

    println!("{}", output)
}

enum Token {
    Identity(String),

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
    Slash,

    Nil,
}

impl Token {
    fn as_str(&self) -> &str {
        match self {
            Token::Identity(s) => s,
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
            Token::Slash => "/",
        }
    }
}
