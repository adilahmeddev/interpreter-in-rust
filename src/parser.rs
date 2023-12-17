use std::fmt::Display;

use crate::token::Token;

struct Program{
}
struct Parser{
    
}

impl Parser{

    fn New(tokens: Vec<Token>)->Self{
        
    }
}

enum NodeType{
    Statement,
    Expression,
}

trait Node:Display { 
    fn TokenLiteral()->String;    
}


