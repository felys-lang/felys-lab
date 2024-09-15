use crate::shared::ast::{ElyChar, ElyFmtChar, ElyPlaceHolder, ElyString};

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
    fn ely_placeholder(&mut self) -> Option<ElyPlaceHolder>;
    fn ely_char(&mut self) -> Option<ElyChar>;
}