use std::fmt::Display;

use crate::token::Token;

struct Program{
}
struct Parser{
    
}

impl Parser{

    fn New(tokens: Vec<Token>, n: NodeType)->Self{
        let _ = match n {
            NodeType::Statement(s) =>{
                s.token_literal()
            }, 
            NodeType::Expression => todo!(),
        }
    }
}

enum NodeType{
    Statement(Statement),
    Expression,
}

trait Node:Display { 
    fn token_literal(&self)->String;    
}

struct Statement{
}

impl Node for Statement {
    fn token_literal(&self)->String {
        todo!()
    }

}

impl Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
