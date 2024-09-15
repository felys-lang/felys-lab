use crate::shared::ast::*;
use crate::expr::registry::*;
use daybreak::Parser;
use crate::string;

impl Base for Parser<'_, CacheType, CacheResult> {
    type CT = CacheType;
    type CR = CacheResult;
}

impl Method for Parser<'_, CacheType, CacheResult> {
    fn ely_expression(&mut self) -> Option<ElyExpression> {
        todo!()
    }
    
    fn ely_disjunction(&mut self) -> Option<ElyDisjunction> {
        todo!()
    }

    fn ely_conjunction(&mut self) -> Option<ElyConjunction> {
        todo!()
    }

    fn ely_inversion(&mut self) -> Option<ElyInversion> {
        todo!()
    }

    fn ely_comparison(&mut self) -> Option<ElyComparison> {
        todo!()
    }

    fn ely_additive(&mut self) -> Option<ElyAdditive> {
        todo!()
    }

    fn ely_multiplicity(&mut self) -> Option<ElyMultiplicity> {
        todo!()
    }

    fn ely_unary(&mut self) -> Option<ElyUnary> {
        todo!()
    }

    fn ely_evaluation(&mut self) -> Option<ElyEvaluation> {
        todo!()
    }

    fn ely_arguments(&mut self) -> Option<ElyArguments> {
        todo!()
    }

    #[daybreak::memoize(ElyPrimary)]
    fn ely_primary(&mut self) -> Option<ElyPrimary> {
        let pos = self.stream.mark();
        let mut cut = false;
        if let Some(result) = || -> Option<ElyPrimary> {
            self.expect("(")?;
            cut = true;
            let expr = self.ely_expression()?;
            self.expect(")")?;
            Some(ElyPrimary::Parentheses(Box::new(expr)))
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        if cut { return None; }
        if let Some(result) = || -> Option<ElyPrimary> {
            let string = string::string(self)?;
            Some(ElyPrimary::String(string))
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        if let Some(result) = || -> Option<ElyPrimary> {
            let boolean = self.ely_boolean()?;
            Some(ElyPrimary::Boolean(boolean))
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        if let Some(result) = || -> Option<ElyPrimary> {
            let ns = self.ely_namespace()?;
            Some(ElyPrimary::Identifier(ns))
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        if let Some(result) = || -> Option<ElyPrimary> {
            let decimal = self.ely_decimal()?;
            Some(ElyPrimary::Decimal(decimal))
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        if let Some(result) = || -> Option<ElyPrimary> {
            let integer = self.ely_integer()?;
            Some(ElyPrimary::Integer(integer))
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        None
    }

    #[daybreak::lecursion(ElyNamespace)]
    fn ely_namespace(&mut self) -> Option<ElyNamespace> {
        let pos = self.stream.mark();
        if let Some(result) = || -> Option<ElyNamespace> {
            let ns = self.ely_namespace()?;
            self.expect("::")?;
            let name = self.ely_name()?;
            Some(ElyNamespace::Space { ns: Box::new(ns), name })
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        if let Some(result) = || -> Option<ElyNamespace> {
            let name = self.ely_name()?;
            Some(ElyNamespace::Name(name))
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        None
    }

    fn ely_name(&mut self) -> Option<ElyName> {
        todo!()
    }

    fn ely_integer(&mut self) -> Option<ElyInteger> {
        todo!()
    }

    fn ely_decimal(&mut self) -> Option<ElyDecimal> {
        todo!()
    }

    fn ely_boolean(&mut self) -> Option<ElyBoolean> {
        todo!()
    }

    #[daybreak::memoize(ElyExpect)]
    fn ely_expect(&mut self, s: &'static str) -> Option<&'static str> {
        self.expect(s)
    }
}
