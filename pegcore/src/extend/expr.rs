use crate::ast::*;
use crate::core::Parser;
use pegmacro::{lecursion, memoize};

impl Parser {
    pub fn expression(&mut self) ->  Option<Expression> {
        self.disjunction()
    }
    
    #[lecursion(cache = Disjunction)]
    pub fn disjunction(&mut self) -> Option<Disjunction> {
        let pos = self.stream.mark();
        if let Some(result) = || -> Option<Disjunction> {
            let lhs = self.disjunction()?;
            self.expect("or")?;
            self.lookahead(|c| c.is_whitespace() || c=='(')?;
            let conj = self.conjunction()?;
            Some(Disjunction::Rec{ lhs: Box::new(lhs), rhs: conj })
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        if let Some(result) = || -> Option<Disjunction> {
            let conj = self.conjunction()?;
            Some(Disjunction::Plain(conj))
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        None
    }
    
    #[lecursion(cache = Conjunction)]
    pub fn conjunction(&mut self) -> Option<Conjunction> {
        let pos = self.stream.mark();
        if let Some(result) = || -> Option<Conjunction> {
            let lhs = self.conjunction()?;
            self.expect("and")?;
            self.lookahead(|c| c.is_whitespace() || c=='(')?;
            let inv = self.inversion()?;
            Some(Conjunction::Rec{ lhs: Box::new(lhs), rhs: inv })
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        if let Some(result) = || -> Option<Conjunction> {
            let inv = self.inversion()?;
            Some(Conjunction::Plain(inv))
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        None
    }
    
    #[memoize(cache = Inversion)]
    pub fn inversion(&mut self) -> Option<Inversion> {
        let pos = self.stream.mark();
        if let Some(result) = || -> Option<Inversion> {
            self.expect("not")?;
            self.lookahead(|c| c.is_whitespace() || c=='(')?;
            let inv = self.inversion()?;
            Some(Inversion::Rec(Box::new(inv)))
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        if let Some(result) = || -> Option<Inversion> {
            let comp = self.comparison()?;
            Some(Inversion::Plain(comp))
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        None
    }

    #[lecursion(cache = Comparison)]
    pub fn comparison(&mut self) -> Option<Comparison> {
        let pos = self.stream.mark();
        if let Some(result) = || -> Option<Comparison> {
            let lhs = self.comparison()?;
            self.expect("==")?;
            let add = self.additive()?;
            Some(Comparison::Rec { lhs: Box::new(lhs), op: ComOp::Eq, rhs: add })
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        if let Some(result) = || -> Option<Comparison> {
            let lhs = self.comparison()?;
            self.expect("!=")?;
            let add = self.additive()?;
            Some(Comparison::Rec { lhs: Box::new(lhs), op: ComOp::Ne, rhs: add })
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        if let Some(result) = || -> Option<Comparison> {
            let lhs = self.comparison()?;
            self.expect(">")?;
            let add = self.additive()?;
            Some(Comparison::Rec { lhs: Box::new(lhs), op: ComOp::Gt, rhs: add })
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        if let Some(result) = || -> Option<Comparison> {
            let lhs = self.comparison()?;
            self.expect("<")?;
            let add = self.additive()?;
            Some(Comparison::Rec { lhs: Box::new(lhs), op: ComOp::Lt, rhs: add })
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        if let Some(result) = || -> Option<Comparison> {
            let lhs = self.comparison()?;
            self.expect(">=")?;
            let add = self.additive()?;
            Some(Comparison::Rec { lhs: Box::new(lhs), op: ComOp::Ge, rhs: add })
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        if let Some(result) = || -> Option<Comparison> {
            let lhs = self.comparison()?;
            self.expect("<=")?;
            let add = self.additive()?;
            Some(Comparison::Rec { lhs: Box::new(lhs), op: ComOp::Le, rhs: add })
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        if let Some(result) = || -> Option<Comparison> {
            let add = self.additive()?;
            Some(Comparison::Plain(add))
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
            Some(Additive::Rec { lhs: Box::new(lhs), op: AddOp::Add, rhs: mul })
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        if let Some(result) = || -> Option<Additive> {
            let lhs = self.additive()?;
            self.expect("-")?;
            let mul = self.multiplicity()?;
            Some(Additive::Rec { lhs: Box::new(lhs), op: AddOp::Sub, rhs: mul })
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        if let Some(result) = || -> Option<Additive> {
            let mul = self.multiplicity()?;
            Some(Additive::Plain(mul))
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
            Some(Multiplicity::Rec { lhs: Box::new(lhs), op: MulOp::Mul, rhs: unary })
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        if let Some(result) = || -> Option<Multiplicity> {
            let lhs = self.multiplicity()?;
            self.expect("/")?;
            let unary = self.unary()?;
            Some(Multiplicity::Rec { lhs: Box::new(lhs), op: MulOp::Div, rhs: unary })
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        if let Some(result) = || -> Option<Multiplicity> {
            let lhs = self.multiplicity()?;
            self.expect("%")?;
            let unary = self.unary()?;
            Some(Multiplicity::Rec { lhs: Box::new(lhs), op: MulOp::Mod, rhs: unary })
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        if let Some(result) = || -> Option<Multiplicity> {
            let unary = self.unary()?;
            Some(Multiplicity::Plain(unary))
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
            Some(Unary::Rec { op: UnaOp::Pos, inner: Box::new(unary) })
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        if let Some(result) = || -> Option<Unary> {
            self.expect("-")?;
            let unary = self.unary()?;
            Some(Unary::Rec { op: UnaOp::Neg, inner: Box::new(unary) })
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        if let Some(result) = || -> Option<Unary> {
            let eval = self.evaluation()?;
            Some(Unary::Plain(eval))
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
        let mut cut = false;
        if let Some(result) = || -> Option<Primary> {
            self.expect("(")?;
            cut = true;
            let expr = self.expression()?;
            self.expect(")")?;
            Some(Primary::Parentheses(Box::new(expr)))
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        if cut { return None; }
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
