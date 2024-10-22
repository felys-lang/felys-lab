use crate::expr::Expr;

pub enum Stmt {
    /// expression with semicolon: `1 + 1;`
    Semi(Expr),
    /// expression: `1 + 1`
    Expr(Expr),
    /// single semicolon: `;`
    Empty,
}
