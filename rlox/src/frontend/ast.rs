// TODO: I am doubting if I should immediatley use an allocator and push the expr on it, that
// returns an exprId. Binary
// would become
// {left: exprId, operator:..., right: ExprId}. This way I don't have to mess with Box, the expr
// won't be spread out in memory,... But I want to see the memory and performance gains, so lets do
// it the naive way first

use anyhow::*;

use crate::frontend::token::TokenType;

// TODO this enum is exactly the same as the token Literal enum. Something's got to go!!
// TODO even the source can go on the allocator. when we need a substring just store the start and
// end point as ints and when they are actually needed just index into the source code and extract
// the slice like that. I think for now I'll just clone the thing and see how much performance I
// gain by the above approach. I don't want to drag the lifetimes around
#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum Literal {
    Str(String),
    Boolean(bool),
    Float(f64),
}

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum Operator {
    Plus,
    Minus,
    Star,
    Slash,
    Equals,
}
impl Operator {
    pub fn from_token_type(ttype: TokenType) -> Result<Self, Error> {
        match ttype {
            TokenType::Plus => Ok(Operator::Plus),
            TokenType::Minus => Ok(Operator::Minus),
            TokenType::Star => Ok(Operator::Star),
            TokenType::Slash => Ok(Operator::Slash),
            _ => Err(anyhow!(
                "[TokenType {:?}] doesn't have a matching operator",
                ttype
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum Expr {
    Binary {
        left: Box<Expr>,
        op: Operator,
        right: Box<Expr>,
    },
    Assign {
        op: Operator,
        value: Box<Expr>,
    },
    Literal {
        value: Literal,
    },
    Unary {
        op: Operator,
        right: Box<Expr>,
    },
    Variable {
        name: String,
    },
    Grouping {
        value: Box<Expr>,
    },
}
