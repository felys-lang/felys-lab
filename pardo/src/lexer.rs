use crate::registry::{Lexer, CR, CT};
use ast::lit::{Bool, Deci, Int, Str};
use ast::pat::Ident;
use daybreak::Parser;

impl Lexer for Parser<CT, CR> {
    fn int(&mut self) -> Option<Int> {
        if let Some(res) = self.alter(|x| {
            x.expect("0x")?;
            x.stream.strict = true;
            x.cut = true;
            let first = x.scan(|c| c.is_ascii_hexdigit())?;
            let mut body = format!("0x{}", first);
            while let Some(ch) = x.scan(|c| c.is_ascii_hexdigit()) {
                body.push(ch)
            }
            let id = x.table.id(body);
            Some(Int::Base16(id))
        }) {
            return res;
        }
        if let Some(res) = self.alter(|x| {
            x.expect("0o")?;
            x.stream.strict = true;
            x.cut = true;
            let first = x.scan(|c| matches!(c, '0'..='7'))?;
            let mut body = format!("0o{}", first);
            while let Some(ch) = x.scan(|c| matches!(c, '0'..='7')) {
                body.push(ch)
            }
            let id = x.table.id(body);
            Some(Int::Base8(id))
        }) {
            return res;
        }
        if let Some(res) = self.alter(|x| {
            x.expect("0b")?;
            x.stream.strict = true;
            x.cut = true;
            let first = x.scan(|c| matches!(c, '0'|'1'))?;
            let mut body = format!("0b{}", first);
            while let Some(ch) = x.scan(|c| matches!(c, '0'|'1')) {
                body.push(ch)
            }
            let id = x.table.id(body);
            Some(Int::Base2(id))
        }) {
            return res;
        }
        if let Some(res) = self.alter(|x| {
            x.expect("0")?;
            x.stream.strict = true;
            x.cut = true;
            x.lookahead(|c| !c.is_ascii_digit())?;
            let id = x.table.id("0".to_string());
            Some(Int::Base10(id))
        }) {
            return res;
        }
        if let Some(res) = self.alter(|x| {
            let first = x.scan(|c| c.is_ascii_digit())?;
            x.stream.strict = true;
            let mut body = String::from(first);
            while let Some(ch) = x.scan(|c| c.is_ascii_digit()) {
                body.push(ch)
            }
            let id = x.table.id(body);
            Some(Int::Base10(id))
        }) {
            return res;
        }
        None
    }

    fn deci(&mut self) -> Option<Deci> {
        if let Some(res) = self.alter(|x| {
            x.expect("0")?;
            x.stream.strict = true;
            x.cut = true;
            x.expect(".")?;
            let first = x.scan(|c| c.is_ascii_digit())?;
            let mut body = format!("0.{}", first);
            while let Some(ch) = x.scan(|c| c.is_ascii_digit()) {
                body.push(ch)
            }
            let id = x.table.id(body);
            Some(id)
        }) {
            return res;
        }
        if let Some(res) = self.alter(|x| {
            let first = x.scan(|c| c.is_ascii_digit())?;
            x.stream.strict = true;
            let mut body = String::from(first);
            while let Some(ch) = x.scan(|c| c.is_ascii_digit()) {
                body.push(ch)
            }
            x.expect(".")?;
            body.push('.');
            let first = x.scan(|c| c.is_ascii_digit())?;
            body.push(first);
            while let Some(ch) = x.scan(|c| c.is_ascii_digit()) {
                body.push(ch)
            }
            let id = x.table.id(body);
            Some(id)
        }) {
            return res;
        }
        None
    }

    fn bool(&mut self) -> Option<Bool> {
        if let Some(res) = self.alter(|x| {
            x.keyword("true")?;
            Some(Bool::True)
        }) {
            return res;
        }
        if let Some(res) = self.alter(|x| {
            x.keyword("false")?;
            Some(Bool::False)
        }) {
            return res;
        }
        None
    }

    fn str(&mut self) -> Option<Str> {
        if let Some(res) = self.alter(|x| {
            x.expect("\"")?;
            x.stream.strict = true;
            x.cut = true;
            let mut body = String::new();
            while let Some(ch) = x.char() {
                body.push(ch);
            }
            x.expect("\"")?;
            let id = x.table.id(body);
            Some(Str::Plain(id))
        }) {
            return res;
        }
        None
    }

    fn char(&mut self) -> Option<char> {
        if let Some(res) = self.alter(|x| {
            let mut ch = x.scan(|c| c != '"')?;
            if ch == '\\' {
                x.stream.strict = true;
                ch = match x.stream.next()? {
                    '\\' => '\\',
                    '\'' => '\'',
                    '\"' => '\"',
                    'r' => '\r',
                    'n' => '\n',
                    't' => '\t',
                    _ => return None
                };
            }
            Some(ch)
        }) {
            return res;
        }
        None
    }

    fn ident(&mut self) -> Option<Ident> {
        if let Some(res) = self.alter(|x| {
            let first = x.scan(|c| c.is_ascii_alphabetic() || c == '_')?;
            x.stream.strict = true;
            let mut body = String::from(first);
            while let Some(ch) = x.scan(|c| c.is_ascii_alphanumeric() || c == '_') {
                body.push(ch)
            }
            let id = x.table.id(body);
            Some(id)
        }) {
            return res;
        }
        None
    }

    fn keyword(&mut self, s: &'static str) -> Option<&'static str> {
        if let Some(res) = self.alter(|x| {
            x.expect(s)?;
            x.stream.strict = true;
            x.lookahead(|c| !c.is_ascii_alphanumeric())?;
            Some(s)
        }) {
            return res;
        }
        None
    }
}
