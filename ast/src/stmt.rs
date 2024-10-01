use crate::expr::Expr;

pub struct Block(Vec<Stmt>);

pub enum Stmt {
    Semi(Expr),
    Expr(Expr),
    Empty,
}