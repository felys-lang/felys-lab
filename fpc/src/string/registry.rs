use crate::shared::ast::*;

#[daybreak::ct]
pub struct CacheType;

#[daybreak::cr]
pub struct CacheResult;

pub trait Base {
    type CT;
    type CR;
}

pub trait Method: Base {
    fn ely_string(&mut self) -> Option<ElyString>;
    fn ely_fmt_char(&mut self) -> Option<ElyFmtChar>;
    fn ely_align(&mut self) -> Option<ElyAlign>;
    fn ely_formatter(&mut self) -> Option<ElyFormatter>;
    fn ely_expression(&mut self) -> Option<ElyExpression>;
    fn ely_integer(&mut self) -> Option<ElyInteger>;
    fn ely_char(&mut self) -> Option<ElyChar>;
}
