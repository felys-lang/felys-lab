use crate::shared::ast::*;

#[daybreak::ct]
pub enum CacheType {
    ElyExpect(&'static str),
    ElyMultiplicity,
    ElyDisjunction,
    ElyConjunction,
    ElyInversion,
    ElyComparison,
    ElyEvaluation,
    ElyNamespace,
    ElyAdditive,
    ElyPrimary,
    ElyBoolean,
    ElyDecimal,
    ElyInteger,
    ElyString,
    ElyUnary,
    ElyName,
}

#[daybreak::cr]
pub enum CacheResult {
    ElyExpect(Option<&'static str>),
    ElyMultiplicity(Option<ElyMultiplicity>),
    ElyComparison(Option<ElyComparison>),
    ElyEvaluation(Option<ElyEvaluation>),
    ElyDisjunction(Option<ElyDisjunction>),
    ElyConjunction(Option<ElyConjunction>),
    ElyInversion(Option<ElyInversion>),
    ElyNamespace(Option<ElyNamespace>),
    ElyAdditive(Option<ElyAdditive>),
    ElyPrimary(Option<ElyPrimary>),
    ElyBoolean(Option<ElyBoolean>),
    ElyDecimal(Option<ElyDecimal>),
    ElyInteger(Option<ElyInteger>),
    ElyString(Option<ElyString>),
    ElyUnary(Option<ElyUnary>),
    ElyName(Option<ElyName>),
}

pub trait Base {
    type CT;
    type CR;
}

pub trait Method: Base {
    fn ely_expression(&mut self) -> Option<ElyExpression>;
    fn ely_disjunction(&mut self) -> Option<ElyDisjunction>;
    fn ely_conjunction(&mut self) -> Option<ElyConjunction>;
    fn ely_inversion(&mut self) -> Option<ElyInversion>;
    fn ely_comparison(&mut self) -> Option<ElyComparison>;
    fn ely_additive(&mut self) -> Option<ElyAdditive>;
    fn ely_multiplicity(&mut self) -> Option<ElyMultiplicity>;
    fn ely_unary(&mut self) -> Option<ElyUnary>;
    fn ely_evaluation(&mut self) -> Option<ElyEvaluation>;
    fn ely_arguments(&mut self) -> Option<ElyArguments>;
    fn ely_primary(&mut self) -> Option<ElyPrimary>;
    fn ely_namespace(&mut self) -> Option<ElyNamespace>;
    fn ely_name(&mut self) -> Option<ElyName>;
    fn ely_integer(&mut self) -> Option<ElyInteger>;
    fn ely_decimal(&mut self) -> Option<ElyDecimal>;
    fn ely_boolean(&mut self) -> Option<ElyBoolean>;
    fn ely_expect(&mut self, s: &'static str) -> Option<&'static str>;
}
