use crate::shared::ast::ElyString;
use crate::string::registry::Method;
use daybreak::Parser;
use std::fmt::{Debug, Display};
use std::hash::Hash;

mod parser;
mod registry;

pub fn parse<CT, CR>(other: &mut Parser<CT, CR>) -> Option<ElyString>
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

#[test]
fn test() {
    let cases = [
        (r#"  "el ysi\ta""#, r#""el ysi\ta""#),
    ];

    for (case, expect) in cases {
        let mut x = Parser::<i32, i32>::new(case);
        let result = parse(&mut x).unwrap().to_string();
        assert_eq!(result.as_str(), expect);
        assert_eq!(x.stream.mark(), case.len())
    }
}
