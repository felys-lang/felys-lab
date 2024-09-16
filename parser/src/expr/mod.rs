use crate::expr::registry::Method;
use crate::shared::ast::ElyPrimary;
use daybreak::Parser;
use std::fmt::{Debug, Display};
use std::hash::Hash;

mod registry;
mod parser;


pub fn expr<CT, CR>(other: &mut Parser<CT, CR>) -> Option<ElyPrimary>
where
    CT: Display + Debug + Hash + PartialEq + Eq + Clone + Copy,
    CR: Display + Debug + Clone,
{
    let mut parser = other.export();
    let result = parser.ely_primary()?;
    let pos = parser.stream.mark();
    other.stream.jump(pos);
    Some(result)
}

#[test]
fn test() {
    let mut x = Parser::<i32, i32>::new("123.0");
    let result = expr(&mut x);
    println!("{:?}", result)
}
