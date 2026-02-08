use crate::frontend::token::Token;

// TODO: I am doubting if I should immediatley use an allocator and push the expr on it, that
// returns an exprId. Binary
// would become
// {left: exprId, operator:..., right: ExprId}. This way I don't have to mess with Box, the expr
// won't be spread out in memory,... But I want to see the memory and performance gains, so lets do
// it the naive way first

enum Operator {
    Plus,
    Minus,
    Star,
    Slash,
}

enum Expr {
    Binary {
        left: Box<Expr>,
        op: Operator,
        right: Box<Expr>,
    },
}
