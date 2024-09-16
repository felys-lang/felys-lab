use crate::expr;
use crate::shared::ast::*;
use crate::string::registry::*;
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
            let expr = self.ely_expression();
            let fmt = self.ely_formatter();
            self.expect("}")?;
            Some(ElyFmtChar::Placeholder { expr, fmt })
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

    fn ely_align(&mut self) -> Option<ElyAlign> {
        let pos = self.stream.mark();
        if let Some(result) = || -> Option<ElyAlign> {
            self.expect("<")?;
            Some(ElyAlign::Left)
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        if let Some(result) = || -> Option<ElyAlign> {
            self.expect(">")?;
            Some(ElyAlign::Right)
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        if let Some(result) = || -> Option<ElyAlign> {
            self.expect("^")?;
            Some(ElyAlign::Middle)
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        None
    }

    fn ely_formatter(&mut self) -> Option<ElyFormatter> {
        let pos = self.stream.mark();
        if let Some(result) = || -> Option<ElyFormatter> {
            self.expect(":")?;
            let debug = self.expect("?").is_some();
            let align = self.ely_align();
            let len = self.ely_integer();
            Some(ElyFormatter { debug, align, len })
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        None
    }

    fn ely_expression(&mut self) -> Option<ElyExpression> {
        expr::expr(self)
    }

    fn ely_integer(&mut self) -> Option<ElyInteger> {
        expr::integer(self)
    }

    fn ely_char(&mut self) -> Option<ElyChar> {
        let pos = self.stream.mark();
        if let Some(result) = || -> Option<ElyChar> {
            let ch = self.scan(|c| !matches!(c, '\\' | '\n' | '\t' | '\r'))?;
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
