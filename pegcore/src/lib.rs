use crate::ast::Program;
use crate::core::*;

mod ast;
mod core;
mod extend;

pub fn parse(code: String) {
    let _ = Parser {
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
}

impl Parser {
    fn program(&mut self) -> Option<Program> {
        let pos = self.stream.mark();
        if let Some(body) = || -> Option<Program> {
            let body = self.integer()?;
            Some(Program(body))
        }() {
            return Some(body);
        } else {
            self.stream.jump(pos)
        }
        None
    }
}

#[test]
fn test() {
    parse("0x123".to_string())
}
