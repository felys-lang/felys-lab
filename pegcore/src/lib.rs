use crate::ast::Program;
use crate::core::*;

mod ast;
mod core;
mod extend;

pub fn parse(code: String) {
    let mut parser = Parser::new(code, true);
    if parser.program().is_none() {
        let leftover = parser.stream.collect::<String>();
        println!("leftover: `{}`", leftover);
    }
}

impl Parser {
    fn program(&mut self) -> Option<Program> {
        let body = self.integer()?;
        let pos = self.stream.mark();
        if self.stream.next().is_none() {
            Some(Program(body))
        } else {
            self.stream.jump(pos);
            None
        }
    }
}

#[test]
fn test() {
    parse("0b111134".to_string())
}
