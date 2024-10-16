use crate::registry::{Lexer, CR, CT};
use ast::lit::{Bool, Deci, Int, Str};
use ast::pat::Ident;
use daybreak::Parser;

impl Lexer for Parser<CT, CR> {
    fn int(&mut self) -> Option<Int> {
        todo!()
    }

    fn deci(&mut self) -> Option<Deci> {
        todo!()
    }

    fn bool(&mut self) -> Option<Bool> {
        todo!()
    }

    fn str(&mut self) -> Option<Str> {
        if let Some(res) = self.alter(|x| {
            x.expect("r\"")?;
            x.stream.strict = true;
            x.cut = true;
            let mut body = String::new();
            while let Some(ch) = x.scan(|c| c != '"') {
                body.push(ch);
            }
            x.expect("\"")?;
            let id = x.table.id(body);
            Some(Str::Raw(id))
        }) {
            return res;
        }
        None
    }

    fn ident(&mut self) -> Option<Ident> {
        if let Some(res) = self.alter(|x| {
            let first = x.scan(|c| c.is_ascii_alphabetic() || c == '_')?;
            x.stream.strict = true;
            let mut body = String::from(first);
            while let Some(more) = x.scan(|c| c.is_ascii_alphanumeric() || c == '_') {
                body.push(more)
            }
            let id = x.table.id(body);
            Some(id)
        }) {
            return res;
        }
        None
    }

    fn keyword(&mut self, s: &'static str) -> Option<&'static str> {
        if let Some(res) = self.alter(|x| {
            x.expect(s)?;
            x.stream.strict = true;
            x.lookahead(|c| !c.is_ascii_alphanumeric())?;
            Some(s)
        }) {
            return res;
        }
        None
    }
}
