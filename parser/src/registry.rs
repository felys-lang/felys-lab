use ast::expr::Expr;
use ast::item::Item;
use ast::lit::{Bool, Float, Int, Lit, Str};
use ast::pat::{Ident, Pat};
use ast::stmt::Stmt;

pub trait Base {
    type CT;
    type CR;
}

pub trait Helper: Base {
    fn keyword(&mut self, s: &'static str) -> Option<&'static str>;
}

pub trait Literal: Base {
    fn lit(&mut self) -> Option<Lit>;
    fn int(&mut self) -> Option<Int>;
    fn float(&mut self) -> Option<Float>;
    fn bool(&mut self) -> Option<Bool>;
    fn str(&mut self) -> Option<Str>;
}

pub trait Expression: Base {
    fn expr(&mut self) -> Option<Expr>;
}

pub trait Statement: Base {
    fn stmt(&mut self) -> Option<Stmt>;
}

pub trait Pattern: Base {
    fn pat(&mut self) -> Option<Pat>;
    fn ident(&mut self) -> Option<Ident>;
}

pub trait Unit: Base {
    fn item(&mut self) -> Option<Item>;
}
