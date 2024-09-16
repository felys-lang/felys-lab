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
        self.ely_disjunction()
    }
    
    #[daybreak::lecursion(ElyDisjunction)]
    fn ely_disjunction(&mut self) -> Option<ElyDisjunction> {
        let pos = self.stream.mark();
        if let Some(result) = || -> Option<ElyDisjunction> {
            let lhs = self.ely_disjunction()?;
            self.ely_keyword("or")?;
            let conj = self.ely_conjunction()?;
            Some(ElyDisjunction::Rec{ lhs: Box::new(lhs), rhs: conj })
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        if let Some(result) = || -> Option<ElyDisjunction> {
            let conj = self.ely_conjunction()?;
            Some(ElyDisjunction::Plain(conj))
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        None
    }

    #[daybreak::lecursion(ElyConjunction)]
    fn ely_conjunction(&mut self) -> Option<ElyConjunction> {
        let pos = self.stream.mark();
        if let Some(result) = || -> Option<ElyConjunction> {
            let lhs = self.ely_conjunction()?;
            self.ely_keyword("and")?;
            let inv = self.ely_inversion()?;
            Some(ElyConjunction::Rec{ lhs: Box::new(lhs), rhs: inv })
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        if let Some(result) = || -> Option<ElyConjunction> {
            let inv = self.ely_inversion()?;
            Some(ElyConjunction::Plain(inv))
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        None
    }

    #[daybreak::memoize(ElyInversion)]
    fn ely_inversion(&mut self) -> Option<ElyInversion> {
        let pos = self.stream.mark();
        if let Some(result) = || -> Option<ElyInversion> {
            self.ely_keyword("not")?;
            let inv = self.ely_inversion()?;
            Some(ElyInversion::Rec(Box::new(inv)))
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        if let Some(result) = || -> Option<ElyInversion> {
            let comp = self.ely_comparison()?;
            Some(ElyInversion::Plain(comp))
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        None
    }

    #[daybreak::strict]
    fn ely_keyword(&mut self, s: &'static str) -> Option<&'static str> {
        let pos = self.stream.mark();
        if let Some(result) = || -> Option<&'static str> {
            self.expect(s)?;
            self.stream.strict(true);
            self.lookahead(|c| !c.is_ascii_alphanumeric())?;
            Some(s)
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        None
    }

    #[daybreak::lecursion(ElyComparison)]
    fn ely_comparison(&mut self) -> Option<ElyComparison> {
        let pos = self.stream.mark();
        if let Some(result) = || -> Option<ElyComparison> {
            let lhs = self.ely_comparison()?;
            self.expect("==")?;
            let add = self.ely_additive()?;
            Some(ElyComparison::Rec { lhs: Box::new(lhs), op: ElyComOp::Eq, rhs: add })
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        if let Some(result) = || -> Option<ElyComparison> {
            let lhs = self.ely_comparison()?;
            self.expect("!=")?;
            let add = self.ely_additive()?;
            Some(ElyComparison::Rec { lhs: Box::new(lhs), op: ElyComOp::Ne, rhs: add })
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        if let Some(result) = || -> Option<ElyComparison> {
            let lhs = self.ely_comparison()?;
            self.expect(">")?;
            let add = self.ely_additive()?;
            Some(ElyComparison::Rec { lhs: Box::new(lhs), op: ElyComOp::Gt, rhs: add })
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        if let Some(result) = || -> Option<ElyComparison> {
            let lhs = self.ely_comparison()?;
            self.expect("<")?;
            let add = self.ely_additive()?;
            Some(ElyComparison::Rec { lhs: Box::new(lhs), op: ElyComOp::Lt, rhs: add })
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        if let Some(result) = || -> Option<ElyComparison> {
            let lhs = self.ely_comparison()?;
            self.expect(">=")?;
            let add = self.ely_additive()?;
            Some(ElyComparison::Rec { lhs: Box::new(lhs), op: ElyComOp::Ge, rhs: add })
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        if let Some(result) = || -> Option<ElyComparison> {
            let lhs = self.ely_comparison()?;
            self.expect("<=")?;
            let add = self.ely_additive()?;
            Some(ElyComparison::Rec { lhs: Box::new(lhs), op: ElyComOp::Le, rhs: add })
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        if let Some(result) = || -> Option<ElyComparison> {
            let add = self.ely_additive()?;
            Some(ElyComparison::Plain(add))
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        None
    }

    #[daybreak::lecursion(ElyAdditive)]
    fn ely_additive(&mut self) -> Option<ElyAdditive> {
        let pos = self.stream.mark();
        if let Some(result) = || -> Option<ElyAdditive> {
            let lhs = self.ely_additive()?;
            self.expect("+")?;
            let mul = self.ely_multiplicity()?;
            Some(ElyAdditive::Rec { lhs: Box::new(lhs), op: ElyAddOp::Add, rhs: mul })
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        if let Some(result) = || -> Option<ElyAdditive> {
            let lhs = self.ely_additive()?;
            self.expect("-")?;
            let mul = self.ely_multiplicity()?;
            Some(ElyAdditive::Rec { lhs: Box::new(lhs), op: ElyAddOp::Sub, rhs: mul })
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        if let Some(result) = || -> Option<ElyAdditive> {
            let mul = self.ely_multiplicity()?;
            Some(ElyAdditive::Plain(mul))
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        None
    }

    #[daybreak::lecursion(ElyMultiplicity)]
    fn ely_multiplicity(&mut self) -> Option<ElyMultiplicity> {
        let pos = self.stream.mark();
        if let Some(result) = || -> Option<ElyMultiplicity> {
            let lhs = self.ely_multiplicity()?;
            self.expect("*")?;
            let unary = self.ely_unary()?;
            Some(ElyMultiplicity::Rec { lhs: Box::new(lhs), op: ElyMulOp::Mul, rhs: unary })
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        if let Some(result) = || -> Option<ElyMultiplicity> {
            let lhs = self.ely_multiplicity()?;
            self.expect("/")?;
            let unary = self.ely_unary()?;
            Some(ElyMultiplicity::Rec { lhs: Box::new(lhs), op: ElyMulOp::Div, rhs: unary })
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        if let Some(result) = || -> Option<ElyMultiplicity> {
            let lhs = self.ely_multiplicity()?;
            self.expect("%")?;
            let unary = self.ely_unary()?;
            Some(ElyMultiplicity::Rec { lhs: Box::new(lhs), op: ElyMulOp::Mod, rhs: unary })
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        if let Some(result) = || -> Option<ElyMultiplicity> {
            let unary = self.ely_unary()?;
            Some(ElyMultiplicity::Plain(unary))
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        None
    }

    #[daybreak::memoize(ElyUnary)]
    fn ely_unary(&mut self) -> Option<ElyUnary> {
        let pos = self.stream.mark();
        if let Some(result) = || -> Option<ElyUnary> {
            self.expect("+")?;
            let unary = self.ely_unary()?;
            Some(ElyUnary::Rec { op: ElyUnaOp::Pos, inner: Box::new(unary) })
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        if let Some(result) = || -> Option<ElyUnary> {
            self.expect("-")?;
            let unary = self.ely_unary()?;
            Some(ElyUnary::Rec { op: ElyUnaOp::Neg, inner: Box::new(unary) })
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        if let Some(result) = || -> Option<ElyUnary> {
            let eval = self.ely_evaluation()?;
            Some(ElyUnary::Plain(eval))
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        None
    }

    #[daybreak::lecursion(ElyEvaluation)]
    fn ely_evaluation(&mut self) -> Option<ElyEvaluation> {
        let pos = self.stream.mark();
        let mut cut = false;
        if let Some(result) = || -> Option<ElyEvaluation> {
            let e = self.ely_evaluation()?;
            self.expect("(")?;
            cut = true;
            let args = self.ely_arguments()?;
            self.expect(")")?;
            Some(ElyEvaluation::Call { ident: Box::new(e), args })
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        if cut { return None; }
        if let Some(result) = || -> Option<ElyEvaluation> {
            let pe = self.ely_evaluation()?;
            self.expect(".")?;
            cut = true;
            let member = self.ely_name()?;
            Some(ElyEvaluation::Member { ident: Box::new(pe), member })
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        if cut { return None; }
        if let Some(result) = || -> Option<ElyEvaluation> {
            let primary = self.ely_primary()?;
            Some(ElyEvaluation::Primary(primary))
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        None
    }

    fn ely_arguments(&mut self) -> Option<ElyArguments> {
        let pos = self.stream.mark();
        if let Some(result) = || -> Option<ElyArguments> {
            let first = match self.ely_expression() {
                Some(expr) => expr,
                None => return Some(Vec::new())
            };
            let mut body = vec![first];
            while self.expect(",").is_some() {
                let expr = self.ely_expression()?;
                body.push(expr)
            }
            Some(body)
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        None
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
            let string = self.ely_string()?;
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

    #[daybreak::memoize(ElyString)]
    fn ely_string(&mut self) -> Option<ElyString> {
        string::string(self)
    }
}
