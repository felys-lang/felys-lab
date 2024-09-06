use crate::ast::Program;
use crate::core::*;

mod ast;
mod core;
mod extend;
mod vb;

pub fn parse(code: String) {
    let mut parser = Parser::new(code, Verbose::Core);
    if parser.program().is_none() {
        let leftover = parser.stream.collect::<String>();
        println!("leftover: \"{}\"", leftover);
    }
}

impl Parser {
    fn program(&mut self) -> Option<Program> {
        let body = self.logical()?;
        let pos = self.stream.mark();
        if self.stream.next().is_none() {
            Some(body)
        } else {
            self.stream.jump(pos);
            None
        }
    }
}

#[test]
fn test() {
    parse("(1).d-0".to_string())
}
