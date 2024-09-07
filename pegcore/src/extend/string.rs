use crate::ast::*;
use crate::core::Parser;
use pegmacro::memoize;

impl Parser {
    #[memoize(cache = ElyString)]
    pub fn string(&mut self) -> Option<ElyString> {
        let pos = self.stream.mark();
        let mut cut = false;
        self.stream.raw = true;
        if let Some(result) = || -> Option<ElyString> {
            self.expect("f")?;
            cut = true;
            self.expect("\"")?;
            let mut string = Vec::new();
            while self.lookahead(|x| x != '"').is_some() {
                let ch = self.fmt_char()?;
                string.push(ch)
            }
            self.expect("\"")?;
            Some(ElyString::Format(string))
        }() {
            self.stream.raw = false;
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        if cut {
            self.stream.raw = false;
            return None;
        }
        if let Some(result) = || -> Option<ElyString> {
            self.expect("r")?;
            cut = true;
            self.expect("\"")?;
            let mut string = String::new();
            while self.lookahead(|x| x != '"').is_some() {
                let ch = self.stream.next()?;
                string.push(ch)
            }
            self.expect("\"")?;
            Some(ElyString::Raw(string))
        }() {
            self.stream.raw = false;
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        if cut {
            self.stream.raw = false;
            return None;
        }
        if let Some(result) = || -> Option<ElyString> {
            self.expect("\"")?;
            let mut string = Vec::new();
            while self.lookahead(|x| x != '"').is_some() {
                let ch = self.char()?;
                string.push(ch)
            }
            self.expect("\"")?;
            Some(ElyString::Plain(string))
        }() {
            self.stream.raw = false;
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        self.stream.raw = false;
        None
    }

    pub fn fmt_char(&mut self) -> Option<ElyFmtChar> {
        let pos = self.stream.mark();
        let mut cut = false;
        if let Some(result) = || -> Option<ElyFmtChar> {
            self.expect("{{")?;
            Some(ElyFmtChar::Plain(ElyChar::Plain('{')))
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        if let Some(result) = || -> Option<ElyFmtChar> {
            self.expect("}}")?;
            Some(ElyFmtChar::Plain(ElyChar::Plain('}')))
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        if let Some(result) = || -> Option<ElyFmtChar> {
            self.expect("{")?;
            cut = true;
            self.stream.raw = false;
            let expr = self.expression()?;
            self.expect("}")?;
            Some(ElyFmtChar::Placeholder(expr))
        }() {
            self.stream.raw = true;
            return Some(result);
        } else {
            self.stream.raw = true;
            self.stream.jump(pos)
        }
        if cut { return None; }
        if let Some(result) = || -> Option<ElyFmtChar> {
            let ch = self.char()?;
            Some(ElyFmtChar::Plain(ch))
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        None
    }

    pub fn char(&mut self) -> Option<ElyChar> {
        let pos = self.stream.mark();
        let mut cut = false;
        if let Some(result) = || -> Option<ElyChar> {
            self.expect("\\")?;
            cut = true;
            match self.stream.next()? {
                '"' => Some(ElyChar::Quotation),
                '\\' => Some(ElyChar::Backslash),
                'r' => Some(ElyChar::Return),
                'n' => Some(ElyChar::NewLine),
                't' => Some(ElyChar::Tab),
                _ => None
            }
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        if cut { return None; }
        if let Some(result) = || -> Option<ElyChar> {
            let ch = self.stream.next()?;
            Some(ElyChar::Plain(ch))
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        None
    }
}
