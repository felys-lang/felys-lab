use crate::ast::Program;
use crate::core::*;

mod ast;
mod core;
mod extend;

pub fn parse(code: String) {
    let result = Parser {
        stream: Stream {
            body: code,
            cursor: 0,
        },
        cache: Cache {
            body: Default::default(),
            verbose: true,
            hit: 0,
        },
    }.program();
    if result.is_none() {
        println!("parsing failed")
    }
}

impl Parser {
    fn program(&mut self) -> Option<Program> {
        let body = self.integer()?;
        if self.stream.next().is_none() {
            Some(Program(body))
        } else {
            None
        }
    }
}

#[test]
fn test() {
    parse("0x123".to_string())
}
