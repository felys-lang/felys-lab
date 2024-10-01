use crate::lit::{Int, Lit};
use crate::pat::{Ident, Pat, Path};
use crate::stmt::Block;

pub enum Expr {
    /// assignment: `x = 42`
    Assign(Pat, AssOp, Box<Expr>),
    /// break the loop: `break elysia;`
    Break(Option<Box<Expr>>),
    /// code block: `{ elysia }`
    Block(Block),
    /// binary operation: `1 + 2`
    Binary(Box<Expr>, BinOp, Box<Expr>),
    /// closure: `|x| { x+1 }`, `|x| x+1`
    Closure(Vec<Ident>, Block),
    /// skip to the next loop: `continue`
    Continue,
    /// function call: `func(1, 2)`
    Call(Box<Expr>, Vec<Expr>),
    /// field: `elysia.mei`, `elysia.0`
    Field(Box<Expr>, Field),
    /// for loop: `for x in array { block }`
    For(Pat, Box<Expr>, Block),
    /// group of things: `(elysia, 11.11)`
    Group(Vec<Expr>),
    /// match: `match x { Elysia => 1, _ => 0 }`
    Match(Box<Expr>, Vec<Arm>),
    /// if statement with optional else: `if expr { block } else { block }`
    If(Box<Expr>, Block, Option<Box<Expr>>),
    /// return value: `return elysia`
    Return(Option<Box<Expr>>),
    /// literals: `"elysia"`, `11.11`, `true`
    Lit(Lit),
    /// loop with not tests: `loop { block }`
    Loop(Block),
    /// path: `firemoth::elysia`
    Path(Path),
    /// explicit precedence: `(1 + 2)`
    Paren(Box<Expr>),
    /// unary operation: `-1`
    Unary(UnaOp, Box<Expr>),
    /// while loop: `while expr { block }`
    While(Box<Expr>, Block),
}

pub struct Arm {
    pub pat: Pat,
    pub action: Box<Expr>,
}

pub enum Field {
    Named(Ident),
    Anonymous(Int),
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

pub enum AssOp {
    AddEq,
    SubEq,
    MulEq,
    DivEq,
    ModEq,
    Eq,
}
