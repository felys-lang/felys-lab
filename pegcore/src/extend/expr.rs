use crate::ast::*;
use crate::core::Parser;
use pegmacro::{lecursion, memoize};

impl Parser {
    #[lecursion(cache = Comparison)]
    pub fn comparison(&mut self) -> Option<Comparison> {
        let pos = self.stream.mark();
        if let Some(result) = || -> Option<Comparison> {
            let lhs = self.comparison()?;
            self.expect("==")?;
            let add = self.additive()?;
            Some(Comparison::Comparison { lhs: Box::new(lhs), op: ComOp::Eq, rhs: add })
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        if let Some(result) = || -> Option<Comparison> {
            let lhs = self.comparison()?;
            self.expect("!=")?;
            let add = self.additive()?;
            Some(Comparison::Comparison { lhs: Box::new(lhs), op: ComOp::Ne, rhs: add })
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        if let Some(result) = || -> Option<Comparison> {
            let lhs = self.comparison()?;
            self.expect(">")?;
            let add = self.additive()?;
            Some(Comparison::Comparison { lhs: Box::new(lhs), op: ComOp::Gt, rhs: add })
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        if let Some(result) = || -> Option<Comparison> {
            let lhs = self.comparison()?;
            self.expect("<")?;
            let add = self.additive()?;
            Some(Comparison::Comparison { lhs: Box::new(lhs), op: ComOp::Lt, rhs: add })
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        if let Some(result) = || -> Option<Comparison> {
            let lhs = self.comparison()?;
            self.expect(">=")?;
            let add = self.additive()?;
            Some(Comparison::Comparison { lhs: Box::new(lhs), op: ComOp::Ge, rhs: add })
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        if let Some(result) = || -> Option<Comparison> {
            let lhs = self.comparison()?;
            self.expect("<=")?;
            let add = self.additive()?;
            Some(Comparison::Comparison { lhs: Box::new(lhs), op: ComOp::Le, rhs: add })
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        if let Some(result) = || -> Option<Comparison> {
            let add = self.additive()?;
            Some(Comparison::Additive(add))
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        None
    }

    #[lecursion(cache = Additive)]
    pub fn additive(&mut self) -> Option<Additive> {
        let pos = self.stream.mark();
        if let Some(result) = || -> Option<Additive> {
            let lhs = self.additive()?;
            self.expect("+")?;
            let mul = self.multiplicity()?;
            Some(Additive::Additive { lhs: Box::new(lhs), op: AddOp::Add, rhs: mul })
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        if let Some(result) = || -> Option<Additive> {
            let lhs = self.additive()?;
            self.expect("-")?;
            let mul = self.multiplicity()?;
            Some(Additive::Additive { lhs: Box::new(lhs), op: AddOp::Sub, rhs: mul })
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        if let Some(result) = || -> Option<Additive> {
            let mul = self.multiplicity()?;
            Some(Additive::Multiplicity(mul))
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        None
    }

    #[lecursion(cache = Multiplicity)]
    pub fn multiplicity(&mut self) -> Option<Multiplicity> {
        let pos = self.stream.mark();
        if let Some(result) = || -> Option<Multiplicity> {
            let lhs = self.multiplicity()?;
            self.expect("*")?;
            let unary = self.unary()?;
            Some(Multiplicity::Multiplicity { lhs: Box::new(lhs), op: MulOp::Mul, rhs: unary })
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        if let Some(result) = || -> Option<Multiplicity> {
            let lhs = self.multiplicity()?;
            self.expect("/")?;
            let unary = self.unary()?;
            Some(Multiplicity::Multiplicity { lhs: Box::new(lhs), op: MulOp::Div, rhs: unary })
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        if let Some(result) = || -> Option<Multiplicity> {
            let lhs = self.multiplicity()?;
            self.expect("%")?;
            let unary = self.unary()?;
            Some(Multiplicity::Multiplicity { lhs: Box::new(lhs), op: MulOp::Mod, rhs: unary })
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        if let Some(result) = || -> Option<Multiplicity> {
            let unary = self.unary()?;
            Some(Multiplicity::Unary(unary))
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        None
    }

    #[memoize(cache = Unary)]
    pub fn unary(&mut self) -> Option<Unary> {
        let pos = self.stream.mark();
        if let Some(result) = || -> Option<Unary> {
            self.expect("+")?;
            let unary = self.unary()?;
            Some(Unary::Unary { op: UnaOp::Pos, inner: Box::new(unary) })
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        if let Some(result) = || -> Option<Unary> {
            self.expect("-")?;
            let unary = self.unary()?;
            Some(Unary::Unary { op: UnaOp::Neg, inner: Box::new(unary) })
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        if let Some(result) = || -> Option<Unary> {
            let eval = self.evaluation()?;
            Some(Unary::Evaluation(eval))
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        None
    }

    #[lecursion(cache = Evaluation)]
    pub fn evaluation(&mut self) -> Option<Evaluation> {
        let pos = self.stream.mark();
        let mut cut = false;
        if let Some(result) = || -> Option<Evaluation> {
            let pe = self.evaluation()?;
            self.expect("(")?;
            cut = true;
            self.expect(")")?;
            Some(Evaluation::Call { ident: Box::new(pe) })
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        if cut { return None; }
        if let Some(result) = || -> Option<Evaluation> {
            let pe = self.evaluation()?;
            self.expect(".")?;
            cut = true;
            let member = self.name()?;
            Some(Evaluation::Member { ident: Box::new(pe), member })
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        if cut { return None; }
        if let Some(result) = || -> Option<Evaluation> {
            let primary = self.primary()?;
            Some(Evaluation::Primary(primary))
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        None
    }

    #[lecursion(cache = Primary)]
    pub fn primary(&mut self) -> Option<Primary> {
        let pos = self.stream.mark();
        if let Some(result) = || -> Option<Primary> {
            let boolean = self.boolean()?;
            Some(Primary::Boolean(boolean))
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        if let Some(result) = || -> Option<Primary> {
            let ns = self.namespace()?;
            Some(Primary::Identifier(ns))
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        if let Some(result) = || -> Option<Primary> {
            let decimal = self.decimal()?;
            Some(Primary::Decimal(decimal))
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        if let Some(result) = || -> Option<Primary> {
            let integer = self.integer()?;
            Some(Primary::Integer(integer))
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        None
    }

    #[lecursion(cache = Namespace)]
    pub fn namespace(&mut self) -> Option<Namespace> {
        let pos = self.stream.mark();
        if let Some(result) = || -> Option<Namespace> {
            let ns = self.namespace()?;
            self.expect("::")?;
            let name = self.name()?;
            Some(Namespace::Space { ns: Box::new(ns), name })
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        if let Some(result) = || -> Option<Namespace> {
            let name = self.name()?;
            Some(Namespace::Name(name))
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        None
    }
}
