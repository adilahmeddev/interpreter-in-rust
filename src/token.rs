pub enum Token {
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
    Let,
    Blank,
}

impl Token {
    pub fn to_string(&self) -> String {
        match self {
            Token::Identity(s) => format!(" {s} "),
            Token::LParen => String::from(" ( "),
            Token::RParen => String::from(" ) "),
            Token::LBrace => String::from(" { "),
            Token::RBrace => String::from(" } "),
            Token::Nil => String::from(" nil "),
            Token::Bang => String::from(" ! "),
            Token::Plus => String::from(" + "),
            Token::Minus => String::from(" - "),
            Token::Assign => String::from(" = "),
            Token::Equal => String::from(" == "),
            Token::NotEqual => String::from(" != "),
            Token::Asterisk => String::from(" * "),
            Token::Slash => String::from(" / "),
            Token::Let => String::from(" let "),
            Token::Blank => String::default(),
        }
    }
}
