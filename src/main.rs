fn main() {
    let input = String::from("{()}/*/* != == =  adil let le");
    let mut output: String = String::default();

    let arr = input.as_bytes();

    let mut pos = 0;
    let mut peek = 1;
    while peek < input.len() {
        let c = arr[pos];
        let n = arr[peek];
        output.push_str(
            match char::from(c) {
                ')' => Token::RParen,
                '{' => Token::LBrace,
                '(' => Token::LParen,
                '}' => Token::RBrace,
                '+' => Token::Plus,
                '-' => Token::Minus,
                '!' if char::from(n) == '=' => {
                    peek += 1;

                    Token::NotEqual
                }
                '=' if char::from(n) == '=' => {
                    peek += 1;
                    Token::Equal
                }
                '!' => Token::Bang,
                '=' => Token::Assign,
                '/' => Token::Slash,
                '*' => Token::Asterisk,
                ' ' => Token::Blank,
                'l' if char::from(n) == 'e'
                    && peek + 1 < arr.len() - 1
                    && char::from(arr[peek + 1]) == 't' =>
                {
                    peek += 2;
                    Token::Let
                }
                tok if tok.is_alphabetic() => loop {
                    let n = char::from(arr[peek]);
                    println!("n {:?}", char::from(n));
                    if n == ' ' || peek == arr.len() - 1 {
                        let mut x = arr[pos..peek].to_vec();
                        if n != ' ' {
                            x.push(arr[peek]);
                        }
                        let out: String = x.iter().clone().map(|z| char::from(*z)).collect();
                        break Token::Identity(out.to_string());
                    }
                    peek += 1;
                },
                _ => Token::Nil,
            }
            .to_string()
            .as_str(),
        );
        pos = peek;
        peek += 1;
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
    Blank,
}

impl Token {
    fn to_string(&self) -> String {
        match self {
            Token::Identity(s) => {
                format!("Identity ({:?})", s)
            }

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
