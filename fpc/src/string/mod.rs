use crate::shared::ast::ElyString;
use crate::string::registry::Method;
use daybreak::{Parser, Verbose};
use std::fmt::{Debug, Display};
use std::hash::Hash;

mod parser;
mod registry;

pub fn string<CT, CR>(other: &mut Parser<CT, CR>) -> Option<ElyString>
where
    CT: Display + Debug + Hash + PartialEq + Eq + Clone + Copy,
    CR: Display + Debug + Clone,
{
    if other.cache.verbose() != Verbose::Off {
        println!("$ string parser starts")
    }
    let mut parser = other.export();
    let result = parser.ely_string();
    if other.cache.verbose() != Verbose::Off {
        println!("$ string parser ends")
    }
    let result = result?;
    let pos = parser.stream.mark();
    other.stream.jump(pos);
    Some(result)
}
