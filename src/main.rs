fn main() {
    let input = String::from("{()}/*/* != == =");
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
                _ => if char::from(*c) == 'l' || char::from(*c) == 'L'{
                        if char::from(*n) == 'e' || char::from(*n) == 'E' && char::from(*n2) =='t' || char::from(*n2) == 'T'{
                            return  Token::Let;

                    }
                        
                }else  {
                            let mut identity = String::default();
                            identity.push(char::from(*c));
                            while let Some([t])= iter.next(){
                                if char::from(*t) != ' '{ 
                                    break;
                                }
                                identity.push(char::from(*t));
                            }

                            Token::Identity(identity)
                        }

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
    Let,
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
