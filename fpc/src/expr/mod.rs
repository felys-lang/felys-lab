use crate::expr::registry::Method;
use crate::shared::ast::{ElyExpression, ElyInteger};
use daybreak::{Parser, Verbose};
use std::fmt::{Debug, Display};
use std::hash::Hash;

mod registry;
mod parser;


pub fn expr<CT, CR>(other: &mut Parser<CT, CR>) -> Option<ElyExpression>
where
    CT: Display + Debug + Hash + PartialEq + Eq + Clone + Copy,
    CR: Display + Debug + Clone,
{
    if other.cache.verbose() != Verbose::Off {
        println!("$ expression parser starts")
    }
    let mut parser = other.export();
    let result = parser.ely_expression();
    if other.cache.verbose() != Verbose::Off {
        println!("$ expression parser ends")
    }
    let result = result?;
    let pos = parser.stream.mark();
    other.stream.jump(pos);
    Some(result)
}

pub fn integer<CT, CR>(other: &mut Parser<CT, CR>) -> Option<ElyInteger>
where
    CT: Display + Debug + Hash + PartialEq + Eq + Clone + Copy,
    CR: Display + Debug + Clone,
{
    if other.cache.verbose() != Verbose::Off {
        println!("$ integer parser starts")
    }
    let mut parser = other.export();
    let result = parser.ely_integer();
    if other.cache.verbose() != Verbose::Off {
        println!("$ integer parser ends")
    }
    let result = result?;
    let pos = parser.stream.mark();
    other.stream.jump(pos);
    Some(result)
}

#[test]
fn test() {
    let code = r#" n * f(n-1) "#;
    let mut x = Parser::<i32, i32>::new(code).v(Verbose::Miss);
    println!("{}", expr(&mut x).unwrap())
}
