use crate::token::Token;


pub struct Lexer {
   pub tokens: Vec<Token>,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let arr = input.as_bytes();
        let mut tokens: Vec<Token> = Vec::new();
        let mut pos = 0;
        let mut peek = 1;
        while peek < input.len() {
            let c = arr[pos];
            let n = arr[peek];
            tokens.push (
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
            );
            pos = peek;
            peek += 1;
        }

        Self {
            tokens: tokens,
        }
    }

    pub fn to_string(&self) -> String {
       let mut out = String::new();

        self.tokens.iter().for_each(|c|{
            out.push_str(c.to_string().as_str())
        });
        out
    }
}
