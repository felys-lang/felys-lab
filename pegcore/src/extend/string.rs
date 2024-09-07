use crate::ast::*;
use crate::core::Parser;

impl Parser {
    pub fn char(&mut self) -> Option<ElyChar> {
        let pos = self.stream.mark();
        let mut cut = false;
        if let Some(result) = || -> Option<ElyChar> {
            self.expect("\\")?;
            cut = true;
            match self.stream.next()? {
                '\'' => Some(ElyChar::SingleQuote),
                '\"' => Some(ElyChar::DoubleQuote),
                '\\' => Some(ElyChar::Backslash),
                'r' => Some(ElyChar::CarriageReturn),
                'b' => Some(ElyChar::Backspace),
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
