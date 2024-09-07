use crate::ast::*;
use crate::core::Parser;
use pegmacro::{lecursion, memoize};

impl Parser {
    pub fn expression(&mut self) ->  Option<ElyExpression> {
        self.disjunction()
    }
    
    #[lecursion(cache = ElyDisjunction)]
    pub fn disjunction(&mut self) -> Option<ElyDisjunction> {
        let pos = self.stream.mark();
        if let Some(result) = || -> Option<ElyDisjunction> {
            let lhs = self.disjunction()?;
            self.expect("or")?;
            self.lookahead(|c| c.is_whitespace() || c=='(')?;
            let conj = self.conjunction()?;
            Some(ElyDisjunction::Rec{ lhs: Box::new(lhs), rhs: conj })
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        if let Some(result) = || -> Option<ElyDisjunction> {
            let conj = self.conjunction()?;
            Some(ElyDisjunction::Plain(conj))
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        None
    }
    
    #[lecursion(cache = ElyConjunction)]
    pub fn conjunction(&mut self) -> Option<ElyConjunction> {
        let pos = self.stream.mark();
        if let Some(result) = || -> Option<ElyConjunction> {
            let lhs = self.conjunction()?;
            self.expect("and")?;
            self.lookahead(|c| c.is_whitespace() || c=='(')?;
            let inv = self.inversion()?;
            Some(ElyConjunction::Rec{ lhs: Box::new(lhs), rhs: inv })
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        if let Some(result) = || -> Option<ElyConjunction> {
            let inv = self.inversion()?;
            Some(ElyConjunction::Plain(inv))
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        None
    }
    
    #[memoize(cache = ElyInversion)]
    pub fn inversion(&mut self) -> Option<ElyInversion> {
        let pos = self.stream.mark();
        if let Some(result) = || -> Option<ElyInversion> {
            self.expect("not")?;
            self.lookahead(|c| c.is_whitespace() || c=='(')?;
            let inv = self.inversion()?;
            Some(ElyInversion::Rec(Box::new(inv)))
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        if let Some(result) = || -> Option<ElyInversion> {
            let comp = self.comparison()?;
            Some(ElyInversion::Plain(comp))
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        None
    }

    #[lecursion(cache = ElyComparison)]
    pub fn comparison(&mut self) -> Option<ElyComparison> {
        let pos = self.stream.mark();
        if let Some(result) = || -> Option<ElyComparison> {
            let lhs = self.comparison()?;
            self.expect("==")?;
            let add = self.additive()?;
            Some(ElyComparison::Rec { lhs: Box::new(lhs), op: ElyComOp::Eq, rhs: add })
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        if let Some(result) = || -> Option<ElyComparison> {
            let lhs = self.comparison()?;
            self.expect("!=")?;
            let add = self.additive()?;
            Some(ElyComparison::Rec { lhs: Box::new(lhs), op: ElyComOp::Ne, rhs: add })
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        if let Some(result) = || -> Option<ElyComparison> {
            let lhs = self.comparison()?;
            self.expect(">")?;
            let add = self.additive()?;
            Some(ElyComparison::Rec { lhs: Box::new(lhs), op: ElyComOp::Gt, rhs: add })
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        if let Some(result) = || -> Option<ElyComparison> {
            let lhs = self.comparison()?;
            self.expect("<")?;
            let add = self.additive()?;
            Some(ElyComparison::Rec { lhs: Box::new(lhs), op: ElyComOp::Lt, rhs: add })
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        if let Some(result) = || -> Option<ElyComparison> {
            let lhs = self.comparison()?;
            self.expect(">=")?;
            let add = self.additive()?;
            Some(ElyComparison::Rec { lhs: Box::new(lhs), op: ElyComOp::Ge, rhs: add })
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        if let Some(result) = || -> Option<ElyComparison> {
            let lhs = self.comparison()?;
            self.expect("<=")?;
            let add = self.additive()?;
            Some(ElyComparison::Rec { lhs: Box::new(lhs), op: ElyComOp::Le, rhs: add })
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        if let Some(result) = || -> Option<ElyComparison> {
            let add = self.additive()?;
            Some(ElyComparison::Plain(add))
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        None
    }

    #[lecursion(cache = ElyAdditive)]
    pub fn additive(&mut self) -> Option<ElyAdditive> {
        let pos = self.stream.mark();
        if let Some(result) = || -> Option<ElyAdditive> {
            let lhs = self.additive()?;
            self.expect("+")?;
            let mul = self.multiplicity()?;
            Some(ElyAdditive::Rec { lhs: Box::new(lhs), op: ElyAddOp::Add, rhs: mul })
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        if let Some(result) = || -> Option<ElyAdditive> {
            let lhs = self.additive()?;
            self.expect("-")?;
            let mul = self.multiplicity()?;
            Some(ElyAdditive::Rec { lhs: Box::new(lhs), op: ElyAddOp::Sub, rhs: mul })
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        if let Some(result) = || -> Option<ElyAdditive> {
            let mul = self.multiplicity()?;
            Some(ElyAdditive::Plain(mul))
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        None
    }

    #[lecursion(cache = ElyMultiplicity)]
    pub fn multiplicity(&mut self) -> Option<ElyMultiplicity> {
        let pos = self.stream.mark();
        if let Some(result) = || -> Option<ElyMultiplicity> {
            let lhs = self.multiplicity()?;
            self.expect("*")?;
            let unary = self.unary()?;
            Some(ElyMultiplicity::Rec { lhs: Box::new(lhs), op: ElyMulOp::Mul, rhs: unary })
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        if let Some(result) = || -> Option<ElyMultiplicity> {
            let lhs = self.multiplicity()?;
            self.expect("/")?;
            let unary = self.unary()?;
            Some(ElyMultiplicity::Rec { lhs: Box::new(lhs), op: ElyMulOp::Div, rhs: unary })
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        if let Some(result) = || -> Option<ElyMultiplicity> {
            let lhs = self.multiplicity()?;
            self.expect("%")?;
            let unary = self.unary()?;
            Some(ElyMultiplicity::Rec { lhs: Box::new(lhs), op: ElyMulOp::Mod, rhs: unary })
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        if let Some(result) = || -> Option<ElyMultiplicity> {
            let unary = self.unary()?;
            Some(ElyMultiplicity::Plain(unary))
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        None
    }

    #[memoize(cache = ElyUnary)]
    pub fn unary(&mut self) -> Option<ElyUnary> {
        let pos = self.stream.mark();
        if let Some(result) = || -> Option<ElyUnary> {
            self.expect("+")?;
            let unary = self.unary()?;
            Some(ElyUnary::Rec { op: ElyUnaOp::Pos, inner: Box::new(unary) })
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        if let Some(result) = || -> Option<ElyUnary> {
            self.expect("-")?;
            let unary = self.unary()?;
            Some(ElyUnary::Rec { op: ElyUnaOp::Neg, inner: Box::new(unary) })
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        if let Some(result) = || -> Option<ElyUnary> {
            let eval = self.evaluation()?;
            Some(ElyUnary::Plain(eval))
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        None
    }

    #[lecursion(cache = ElyEvaluation)]
    pub fn evaluation(&mut self) -> Option<ElyEvaluation> {
        let pos = self.stream.mark();
        let mut cut = false;
        if let Some(result) = || -> Option<ElyEvaluation> {
            let e = self.evaluation()?;
            self.expect("(")?;
            cut = true;
            let args = self.arguments()?;
            self.expect(")")?;
            Some(ElyEvaluation::Call { ident: Box::new(e), args })
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        if cut { return None; }
        if let Some(result) = || -> Option<ElyEvaluation> {
            let pe = self.evaluation()?;
            self.expect(".")?;
            cut = true;
            let member = self.name()?;
            Some(ElyEvaluation::Member { ident: Box::new(pe), member })
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        if cut { return None; }
        if let Some(result) = || -> Option<ElyEvaluation> {
            let primary = self.primary()?;
            Some(ElyEvaluation::Primary(primary))
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        None
    }
    
    pub fn arguments(&mut self) -> Option<ElyArguments> {
        let pos = self.stream.mark();
        if let Some(result) = || -> Option<ElyArguments> {
            let first = match self.expression() {
                Some(expr) => expr,
                None => return Some(Vec::new())
            };
            let mut body = vec![first];
            while self.expect(",").is_some() {
                let expr = self.expression()?;
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

    #[lecursion(cache = ElyPrimary)]
    pub fn primary(&mut self) -> Option<ElyPrimary> {
        let pos = self.stream.mark();
        let mut cut = false;
        if let Some(result) = || -> Option<ElyPrimary> {
            self.expect("(")?;
            cut = true;
            let expr = self.expression()?;
            self.expect(")")?;
            Some(ElyPrimary::Parentheses(Box::new(expr)))
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        if cut { return None; }
        if let Some(result) = || -> Option<ElyPrimary> {
            let string = self.string()?;
            Some(ElyPrimary::String(string))
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        if let Some(result) = || -> Option<ElyPrimary> {
            let boolean = self.boolean()?;
            Some(ElyPrimary::Boolean(boolean))
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        if let Some(result) = || -> Option<ElyPrimary> {
            let ns = self.namespace()?;
            Some(ElyPrimary::Identifier(ns))
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        if let Some(result) = || -> Option<ElyPrimary> {
            let decimal = self.decimal()?;
            Some(ElyPrimary::Decimal(decimal))
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        if let Some(result) = || -> Option<ElyPrimary> {
            let integer = self.integer()?;
            Some(ElyPrimary::Integer(integer))
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        None
    }

    #[lecursion(cache = ElyNamespace)]
    pub fn namespace(&mut self) -> Option<ElyNamespace> {
        let pos = self.stream.mark();
        if let Some(result) = || -> Option<ElyNamespace> {
            let ns = self.namespace()?;
            self.expect("::")?;
            let name = self.name()?;
            Some(ElyNamespace::Space { ns: Box::new(ns), name })
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        if let Some(result) = || -> Option<ElyNamespace> {
            let name = self.name()?;
            Some(ElyNamespace::Name(name))
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        None
    }
}
