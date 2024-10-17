use ast::lit::{Bool, Deci, Int, Str};
use ast::pat::Ident;

pub struct CT;

pub struct CR;

pub trait Base {
    type CT;
    type CR;
}

pub trait Lexer: Base {
    fn int(&mut self) -> Option<Int>;
    fn deci(&mut self) -> Option<Deci>;
    fn bool(&mut self) -> Option<Bool>;
    fn str(&mut self) -> Option<Str>;
    fn char(&mut self) -> Option<char>;
    fn ident(&mut self) -> Option<Ident>;
    fn keyword(&mut self, s: &'static str) -> Option<&'static str>;
}
