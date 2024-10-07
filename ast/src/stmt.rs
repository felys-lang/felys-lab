use crate::expr::Expr;

pub type Block = Vec<Stmt>;

pub enum Stmt {
    /// expression with semicolon: `1 + 1;`
    Semi(Expr),
    /// expression: `1 + 1`
    Expr(Expr),
    /// no expression: `;`
    Empty,
}
