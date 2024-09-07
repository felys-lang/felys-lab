use crate::ast::*;
use crate::core::Parser;
use pegmacro::memoize;

impl Parser {
    #[memoize(cache = ElyName)]
    pub fn name(&mut self) -> Option<ElyName> {
        let pos = self.stream.mark();
        if let Some(body) = || -> Option<ElyName> {
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

    #[memoize(cache = ElyInteger)]
    pub fn integer(&mut self) -> Option<ElyInteger> {
        let pos = self.stream.mark();
        let mut cut = false;
        if let Some(body) = || -> Option<ElyInteger> {
            self.expect("0x")?;
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

    #[memoize(cache = ElyDecimal)]
    pub fn decimal(&mut self) -> Option<ElyDecimal> {
        let pos = self.stream.mark();
        let mut cut = false;
        if let Some(body) = || -> Option<ElyDecimal> {
            self.expect("0")?;
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

    #[memoize(cache = ElyBoolean)]
    pub fn boolean(&mut self) -> Option<ElyBoolean> {
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
}
