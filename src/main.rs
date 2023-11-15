fn main() {
    let input = String::from("{()}/*/* != == =  adil ");
    let mut iter = input.as_bytes().windows(3);
    let mut output: String = String::default();

    while let Some([c, n, n2]) = iter.next() {
        if char::from(*c) == ' ' {
            continue;
        }

        output.push_str(
            match char::from(*c) {
                ')' => Token::RParen,
                '{' => Token::LBrace,
                '(' => Token::LParen,
                '}' => Token::RBrace,
                '+' => Token::Plus,
                '-' => Token::Minus,
                '!' => match char::from(*n) == '=' {
                    true => {
                        iter.next();
                        Token::NotEqual
                    }
                    false => Token::Bang,
                },
                '=' => match char::from(*n) == '=' {
                    true => {
                        iter.next();
                        Token::Equal
                    }
                    false => Token::Assign,
                },
                '/' => Token::Slash,
                '*' => Token::Asterisk,
                _ => {
                    if (char::from(*c) == 'l' || char::from(*c) == 'L')
                            && (char::from(*n) == 'e' || char::from(*n) == 'E')
                            && (char::from(*n2) == 't' || char::from(*n2) == 'T')
                    {
                        Token::Let
                    } else {
                        let mut identity = String::default();
                        identity.push(char::from(*c));
                        identity.push(char::from(*n));
                        identity.push(char::from(*n2));
                        while let Some([_x, _y, z]) = iter.next() {
                            if char::from(*z) == ' ' {
                                break;
                            }
                            identity.push(char::from(*z))
                        }
                        Token::Identity(identity)
                    }
                }
            }
            .to_string()
            .as_str(),
        )
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
    Let,
}

impl Token {
    fn to_string(&self) -> String {
        match self {
            Token::Identity(s) => {
                format!("Identity ({:?})", s)
            }

            Token::LParen => String::from("("),
            Token::RParen => String::from(")"),
            Token::LBrace => String::from("{"),
            Token::RBrace => String::from("}"),
            Token::Nil => String::from("nil"),
            Token::Bang => String::from("!"),
            Token::Plus => String::from("+"),
            Token::Minus => String::from("-"),
            Token::Assign => String::from("="),
            Token::Equal => String::from("=="),
            Token::NotEqual => String::from("!="),
            Token::Asterisk => String::from("*"),
            Token::Slash => String::from("/"),
            Token::Let => String::from("let"),
        }
    }
}
