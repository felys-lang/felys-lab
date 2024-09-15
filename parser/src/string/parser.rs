use crate::shared::ast::{ElyChar, ElyFmtChar, ElyPlaceHolder, ElyString};
use crate::string::registry::{Base, CacheResult, CacheType, Method};
use daybreak::Parser;

impl Base for Parser<'_, CacheType, CacheResult> {
    type CT = CacheType;
    type CR = CacheResult;
}

impl Method for Parser<'_, CacheType, CacheResult> {
    #[daybreak::strict]
    fn ely_string(&mut self) -> Option<ElyString> {
        let pos = self.stream.mark();
        let mut cut = false;
        if let Some(result) = || -> Option<ElyString> {
            self.expect("f")?;
            self.stream.strict(true);
            cut = true;
            self.expect("\"")?;
            let mut string = Vec::new();
            while self.lookahead(|x| x != '"').is_some() {
                let ch = self.ely_fmt_char()?;
                string.push(ch)
            }
            self.expect("\"")?;
            Some(ElyString::Format(string))
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        if cut { return None; }
        if let Some(result) = || -> Option<ElyString> {
            self.expect("r")?;
            self.stream.strict(true);
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
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        if cut { return None; }
        if let Some(result) = || -> Option<ElyString> {
            self.expect("\"")?;
            self.stream.strict(true);
            let mut string = Vec::new();
            while self.lookahead(|x| x != '"').is_some() {
                let ch = self.ely_char()?;
                string.push(ch)
            }
            self.expect("\"")?;
            Some(ElyString::Plain(string))
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        None
    }

    #[daybreak::strict]
    fn ely_fmt_char(&mut self) -> Option<ElyFmtChar> {
        let pos = self.stream.mark();
        let mut cut = false;
        if let Some(result) = || -> Option<ElyFmtChar> {
            self.expect("{{")?;
            Some(ElyFmtChar::Open)
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        if let Some(result) = || -> Option<ElyFmtChar> {
            self.expect("}}")?;
            Some(ElyFmtChar::Close)
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        if let Some(result) = || -> Option<ElyFmtChar> {
            self.expect("{")?;
            self.stream.strict(false);
            cut = true;
            let placeholder = self.ely_placeholder()?;
            self.expect("}")?;
            Some(ElyFmtChar::Placeholder(placeholder))
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        if cut { return None; }
        if let Some(result) = || -> Option<ElyFmtChar> {
            let ch = self.ely_char()?;
            Some(ElyFmtChar::Plain(ch))
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        None
    }

    fn ely_placeholder(&mut self) -> Option<ElyPlaceHolder> {
        todo!()
    }

    fn ely_char(&mut self) -> Option<ElyChar> {
        let pos = self.stream.mark();
        if let Some(result) = || -> Option<ElyChar> {
            let ch = self.scan(|c| !matches!(c, '\\' | '\n' | '\t' | 'r'))?;
            Some(ElyChar::Plain(ch))
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        if let Some(result) = || -> Option<ElyChar> {
            self.expect("\\\\")?;
            Some(ElyChar::Backslash)
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        if let Some(result) = || -> Option<ElyChar> {
            self.expect("\\\"")?;
            Some(ElyChar::Quotation)
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        if let Some(result) = || -> Option<ElyChar> {
            self.expect("\\n")?;
            Some(ElyChar::NewLine)
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        if let Some(result) = || -> Option<ElyChar> {
            self.expect("\\r")?;
            Some(ElyChar::Return)
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        if let Some(result) = || -> Option<ElyChar> {
            self.expect("\\t")?;
            Some(ElyChar::Tab)
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        None
    }
}
