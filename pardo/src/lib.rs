use crate::registry::{Base, CR, CT};
use daybreak::Parser;

mod lexer;
mod registry;

impl Base for Parser<CT, CR> {
    type CT = CT;
    type CR = CR;
}
