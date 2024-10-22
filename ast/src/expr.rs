use crate::lit::Lit;
use crate::pat::{Ident, Pat, Path};
use crate::stmt::Stmt;
use utils::P;

pub enum Expr {
    /// array with data type: `[1, 2, 3]`
    Array(Vec<Expr>),
    /// assignment: `x = 42`
    Assign(Pat, AssOp, P<Expr>),
    /// break the loop: `break elysia;`
    Break(Option<P<Expr>>),
    /// code block: `{ elysia }`
    Block(Vec<Stmt>),
    /// binary operation: `1 + 2`
    Binary(P<Expr>, BinOp, P<Expr>),
    /// closure: `|x| { x+1 }`, `|x| x+1`
    Closure(Vec<Ident>, Vec<Stmt>),
    /// skip to the next loop: `continue`
    Continue,
    /// function call: `func(1, 2)`
    Call(P<Expr>, Vec<Expr>),
    /// field: `elysia.mei`, `elysia.0`
    Field(P<Expr>, Ident),
    /// for loop: `for x in array { block }`
    For(Pat, P<Expr>, Vec<Stmt>),
    /// group of things: `(elysia, 11.11)`
    Group(Vec<Expr>),
    /// match: `match x { Elysia => 1, _ => 0 }`
    Match(P<Expr>, Vec<Arm>),
    /// if statement with optional else: `if expr { block } else { block }`
    If(P<Expr>, Vec<Stmt>, Option<P<Expr>>),
    /// literals: `"elysia"`, `11.11`, `true`
    Lit(Lit),
    /// loop with not tests: `loop { block }`
    Loop(Vec<Stmt>),
    /// path: `firemoth::elysia`
    Path(Path),
    /// explicit precedence: `(1 + 2)`
    Paren(P<Expr>),
    /// return value: `return elysia`
    Return(Option<P<Expr>>),
    /// unary operation: `-1`
    Unary(UnaOp, P<Expr>),
    /// while loop: `while expr { block }`
    While(P<Expr>, Vec<Stmt>),
}

pub struct Arm {
    pub pat: Pat,
    pub action: Expr,
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
