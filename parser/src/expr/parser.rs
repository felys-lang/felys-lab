use crate::expr::registry::*;
use crate::shared::ast::*;
use crate::string;
use daybreak::Parser;

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

    #[daybreak::strict]
    #[daybreak::memoize(ElyName)]
    fn ely_name(&mut self) -> Option<ElyName> {
        let pos = self.stream.mark();
        if let Some(body) = || -> Option<ElyName> {
            let first = self.scan(|c| c.is_ascii_alphabetic() || c == '_')?;
            self.stream.strict(true);
            let mut body = String::from(first);
            while let Some(more) = self.scan(|c| c.is_ascii_alphanumeric() || c == '_') {
                body.push(more)
            }
            Some(body)
        }() {
            return Some(body);
        } else {
            self.stream.jump(pos)
        }
        None
    }

    #[daybreak::strict]
    #[daybreak::memoize(ElyInteger)]
    fn ely_integer(&mut self) -> Option<ElyInteger> {
        let pos = self.stream.mark();
        let mut cut = false;
        if let Some(body) = || -> Option<ElyInteger> {
            self.expect("0x")?;
            self.stream.strict(true);
            cut = true;
            let first = self.scan(|c| c.is_ascii_hexdigit())?;
            let mut body = String::from(first);
            while let Some(more) = self.scan(|c| c.is_ascii_hexdigit()) {
                body.push(more)
            }
            Some(ElyInteger::Base16(body))
        }() {
            return Some(body);
        } else {
            self.stream.jump(pos)
        }
        if cut { return None; }
        if let Some(body) = || -> Option<ElyInteger> {
            self.expect("0o")?;
            self.stream.strict(true);
            cut = true;
            let first = self.scan(|c| matches!(c, '0'..='7'))?;
            let mut body = String::from(first);
            while let Some(more) = self.scan(|c| matches!(c, '0'..='7')) {
                body.push(more)
            }
            Some(ElyInteger::Base8(body))
        }() {
            return Some(body);
        } else {
            self.stream.jump(pos)
        }
        if cut { return None; }
        if let Some(body) = || -> Option<ElyInteger> {
            self.expect("0b")?;
            self.stream.strict(true);
            cut = true;
            let first = self.scan(|c| matches!(c, '0'|'1'))?;
            let mut body = String::from(first);
            while let Some(more) = self.scan(|c| matches!(c, '0'|'1')) {
                body.push(more)
            }
            Some(ElyInteger::Base2(body))
        }() {
            return Some(body);
        } else {
            self.stream.jump(pos)
        }
        if cut { return None; }
        if let Some(body) = || -> Option<ElyInteger> {
            self.expect("0")?;
            cut = true;
            self.lookahead(|c| !c.is_ascii_digit())?;
            Some(ElyInteger::Base10("0".to_string()))
        }() {
            return Some(body);
        } else {
            self.stream.jump(pos)
        }
        if cut { return None; }
        if let Some(body) = || -> Option<ElyInteger> {
            let first = self.scan(|c| c.is_ascii_digit())?;
            self.stream.strict(true);
            let mut body = String::from(first);
            while let Some(more) = self.scan(|c| c.is_ascii_digit()) {
                body.push(more)
            }
            Some(ElyInteger::Base10(body))
        }() {
            return Some(body);
        } else {
            self.stream.jump(pos)
        }
        None
    }

    #[daybreak::strict]
    #[daybreak::memoize(ElyDecimal)]
    fn ely_decimal(&mut self) -> Option<ElyDecimal> {
        let pos = self.stream.mark();
        let mut cut = false;
        if let Some(body) = || -> Option<ElyDecimal> {
            self.expect("0")?;
            self.stream.strict(true);
            cut = true;
            self.expect(".")?;
            let first = self.scan(|c| c.is_ascii_digit())?;
            let mut frac = String::from(first);
            while let Some(more) = self.scan(|c| c.is_ascii_digit()) {
                frac.push(more)
            }
            Some(ElyDecimal { whole: "0".to_string(), frac })
        }() {
            return Some(body);
        } else {
            self.stream.jump(pos)
        }
        if cut { return None; }
        if let Some(body) = || -> Option<ElyDecimal> {
            let first = self.scan(|c| c.is_ascii_digit())?;
            self.stream.strict(true);
            let mut whole = String::from(first);
            while let Some(more) = self.scan(|c| c.is_ascii_digit()) {
                whole.push(more)
            }
            self.expect(".")?;
            let first = self.scan(|c| c.is_ascii_digit())?;
            let mut frac = String::from(first);
            while let Some(more) = self.scan(|c| c.is_ascii_digit()) {
                frac.push(more)
            }
            Some(ElyDecimal { whole, frac })
        }() {
            return Some(body);
        } else {
            self.stream.jump(pos)
        }
        None
    }

    #[daybreak::memoize(ElyBoolean)]
    fn ely_boolean(&mut self) -> Option<ElyBoolean> {
        let pos = self.stream.mark();
        if let Some(body) = || -> Option<ElyBoolean> {
            self.expect("true")?;
            Some(ElyBoolean::True)
        }() {
            return Some(body);
        } else {
            self.stream.jump(pos)
        }
        if let Some(body) = || -> Option<ElyBoolean> {
            self.expect("false")?;
            Some(ElyBoolean::False)
        }() {
            return Some(body);
        } else {
            self.stream.jump(pos)
        }
        None
    }

    #[daybreak::memoize(ElyExpect)]
    fn ely_expect(&mut self, s: &'static str) -> Option<&'static str> {
        self.expect(s)
    }
}
