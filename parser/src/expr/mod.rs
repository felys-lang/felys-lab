use crate::expr::registry::Method;
use crate::shared::ast::{ElyExpression, ElyInteger};
use daybreak::Parser;
use std::fmt::{Debug, Display};
use std::hash::Hash;

mod registry;
mod parser;


pub fn expr<CT, CR>(other: &mut Parser<CT, CR>) -> Option<ElyExpression>
where
    CT: Display + Debug + Hash + PartialEq + Eq + Clone + Copy,
    CR: Display + Debug + Clone,
{
    let mut parser = other.export();
    let result = parser.ely_expression()?;
    let pos = parser.stream.mark();
    other.stream.jump(pos);
    Some(result)
}

pub fn integer<CT, CR>(other: &mut Parser<CT, CR>) -> Option<ElyInteger>
where
    CT: Display + Debug + Hash + PartialEq + Eq + Clone + Copy,
    CR: Display + Debug + Clone,
{
    let mut parser = other.export();
    let result = parser.ely_integer()?;
    let pos = parser.stream.mark();
    other.stream.jump(pos);
    Some(result)
}

#[test]
fn test() {
    expr(&mut Parser::<i32, i32>::new(r#" f"123{:<9}".format("hi") "#));
}
