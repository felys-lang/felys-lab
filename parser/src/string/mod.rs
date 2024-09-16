use crate::shared::ast::ElyString;
use crate::string::registry::Method;
use daybreak::Parser;
use std::fmt::{Debug, Display};
use std::hash::Hash;

mod parser;
mod registry;

pub fn string<CT, CR>(other: &mut Parser<CT, CR>) -> Option<ElyString>
where
    CT: Display + Debug + Hash + PartialEq + Eq + Clone + Copy,
    CR: Display + Debug + Clone,
{
    let mut parser = other.export();
    let result = parser.ely_string()?;
    let pos = parser.stream.mark();
    other.stream.jump(pos);
    Some(result)
}
