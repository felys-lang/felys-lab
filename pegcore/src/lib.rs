use crate::ast::ElyProgram;
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
        println!("leftover: {}", leftover);
    }
}

impl Parser {
    fn program(&mut self) -> Option<ElyProgram> {
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
    parse(r#"hello(f"{ name }\t"+"12") == false"#.to_string())
}
