use crate::ast::*;
use crate::core::Parser;
use pegmacro::lecursion;

impl Parser {
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
            let eval = self.evaluation()?;
            Some(Primary::Evaluation(eval))
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
        if cut { return None }
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
        if cut { return None }
        if let Some(result) = || -> Option<Evaluation> {
            let ns = self.namespace()?;
            Some(Evaluation::Namespace(ns))
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
