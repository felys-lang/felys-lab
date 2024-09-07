use crate::ast::Program;
use crate::core::*;

mod ast;
mod core;
mod extend;
mod vb;

pub fn parse(code: String) {
    let mut parser = Parser::new(code, Verbose::Core);
    let result = parser.program();
    if parser.cache.verbose >= Verbose::Core {
        println!("cached {} results with {} hits", parser.cache.body.len(), parser.cache.hit);
    }
    if result.is_none() {
        let leftover = parser.stream.collect::<String>();
        println!("leftover: \"{}\"", leftover);
    }
}

impl Parser {
    fn program(&mut self) -> Option<Program> {
        let body = self.expression()?;
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
    parse("((5*3) > 10) and ((4+6) == 11)".to_string())
}
