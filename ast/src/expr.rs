use crate::lit::Lit;
use crate::pat::Pat;
use crate::stmt::Block;

pub enum Expr {
    /// block: `{ elysia }`
    Block(Block),
    /// binary operation: `1 + 2`
    Binary(Box<Expr>, BinOp, Box<Expr>),
    /// function call: `func(1, 2)`
    Call(Box<Expr>, Vec<Expr>),
    /// for loop: `for x in array { block }`
    For(Pat, Box<Expr>, Block),
    /// mutable tuple: `(elysia, 11.11)`
    Group(Vec<Expr>),
    /// if statement with optional else: `if expr { block } else { block }`
    If(Box<Expr>, Block, Option<Box<Expr>>),
    /// literals: `"elysia"`, `11.11`, `true`
    Lit(Lit),
    /// loop with not tests: `loop { block }`
    Loop(Block),
    /// explicit precedence: `(1 + 2)`
    Paren(Box<Expr>),
    /// unary operation: `-1`
    Unary(UnaOp, Box<Expr>),
    /// while loop: `while expr { block }`
    While(Box<Expr>, Block),
}

pub enum BinOp {
    Or,
    And,
    Not,
    Gt,
    Ge,
    Lt,
    Le,
    Eq,
    Ne,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
}

pub enum UnaOp {
    Pos,
    Neg,
}
