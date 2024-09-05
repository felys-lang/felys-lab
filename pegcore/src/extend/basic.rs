use crate::ast::*;
use crate::core::Parser;
use pegmacro::memoize;

impl Parser {
    #[memoize(cache = Name)]
    pub fn name(&mut self) -> Option<Name> {
        let pos = self.stream.mark();
        if let Some(body) = || -> Option<Name> {
            let first = self.scan(|c| c.is_ascii_alphabetic() || c == '_')?;
            let mut body = String::from(first);
            while let Some(more) = self.scan(|c| c.is_ascii_alphanumeric() || c == '_') {
                body.push(more)
            }
            Some(Name(body))
        }() {
            return Some(body);
        } else {
            self.stream.jump(pos)
        }
        None
    }

    #[memoize(cache = Integer)]
    pub fn integer(&mut self) -> Option<Integer> {
        let pos = self.stream.mark();
        let mut cut = false;
        if let Some(body) = || -> Option<Integer> {
            self.expect("0")?;
            cut = true;
            self.expect("x")?;
            let first = self.scan(|c| c.is_ascii_hexdigit())?;
            let mut body = String::from(first);
            while let Some(more) = self.scan(|c| c.is_ascii_hexdigit()) {
                body.push(more)
            }
            Some(Integer::Base16(body))
        }() {
            return Some(body);
        } else if cut {
            self.stream.jump(pos)
        }
        if cut { return None; }
        if let Some(body) = || -> Option<Integer> {
            let first = self.scan(|c| c.is_ascii_digit())?;
            let mut body = String::from(first);
            while let Some(more) = self.scan(|c| c.is_ascii_digit()) {
                body.push(more)
            }
            Some(Integer::Base10(body))
        }() {
            return Some(body);
        } else {
            self.stream.jump(pos)
        }
        None
    }

    #[memoize(cache = Decimal)]
    pub fn decimal(&mut self) -> Option<Decimal> {
        let pos = self.stream.mark();
        if let Some(body) = || -> Option<Decimal> {
            self.nl('0')?;
            let first = self.scan(|c| c.is_ascii_digit())?;
            let mut before = String::from(first);
            while let Some(more) = self.scan(|c| c.is_ascii_digit()) {
                before.push(more)
            }
            self.expect(".")?;
            let first = self.scan(|c| c.is_ascii_digit())?;
            let mut after = String::from(first);
            while let Some(more) = self.scan(|c| c.is_ascii_digit()) {
                after.push(more)
            }
            let formatted = format!("{}.{}", before, after);
            Some(Decimal(formatted))
        }() {
            return Some(body);
        } else {
            self.stream.jump(pos)
        }
        None
    }

    #[memoize(cache = Boolean)]
    pub fn boolean(&mut self) -> Option<Boolean> {
        let pos = self.stream.mark();
        if let Some(body) = || -> Option<Boolean> {
            self.expect("true")?;
            Some(Boolean::True)
        }() {
            return Some(body);
        } else {
            self.stream.jump(pos)
        }
        if let Some(body) = || -> Option<Boolean> {
            self.expect("false")?;
            Some(Boolean::False)
        }() {
            return Some(body);
        } else {
            self.stream.jump(pos)
        }
        None
    }
}