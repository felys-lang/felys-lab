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
            Some(body)
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
            self.expect("0x")?;
            cut = true;
            let first = self.scan(|c| c.is_ascii_hexdigit())?;
            let mut body = String::from(first);
            while let Some(more) = self.scan(|c| c.is_ascii_hexdigit()) {
                body.push(more)
            }
            Some(Integer::Base16(body))
        }() {
            return Some(body);
        } else {
            self.stream.jump(pos)
        }
        if cut { return None; }
        if let Some(body) = || -> Option<Integer> {
            self.expect("0o")?;
            cut = true;
            let first = self.scan(|c| matches!(c, '0'..='7'))?;
            let mut body = String::from(first);
            while let Some(more) = self.scan(|c| matches!(c, '0'..='7')) {
                body.push(more)
            }
            Some(Integer::Base8(body))
        }() {
            return Some(body);
        } else {
            self.stream.jump(pos)
        }
        if cut { return None; }
        if let Some(body) = || -> Option<Integer> {
            self.expect("0b")?;
            cut = true;
            let first = self.scan(|c| matches!(c, '0'|'1'))?;
            let mut body = String::from(first);
            while let Some(more) = self.scan(|c| matches!(c, '0'|'1')) {
                body.push(more)
            }
            Some(Integer::Base2(body))
        }() {
            return Some(body);
        } else {
            self.stream.jump(pos)
        }
        if cut { return None; }
        if let Some(body) = || -> Option<Integer> {
            self.expect("0")?;
            cut = true;
            self.lookahead(|c| !c.is_ascii_digit())?;
            Some(Integer::Base10("0".to_string()))
        }() {
            return Some(body);
        } else {
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
            let first = self.scan(|c| c.is_ascii_digit())?;
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
            Some(Decimal { whole, frac })
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
