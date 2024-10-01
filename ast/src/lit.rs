use crate::expr::Expr;
use crate::Span;

pub enum Lit {
    /// integer: `0xf`, `0o77`, `15`, `0b1111`
    Int(Int),
    /// decimal: `11.11`
    Deci(Deci),
    /// boolean: `true`, `false`
    Bool(Bool),
    /// string: `"elysia"`, `f"{1+1} = 2"`, `r"\t\r\n"`
    Str(Str),
}

pub enum Int {
    Base16(Span),
    Base10(Span),
    Base8(Span),
    Base2(Span),
}

pub struct Deci {
    pub whole: Span,
    pub frac: Span,
}

pub enum Bool {
    True,
    False,
}

pub enum Str {
    Plain(PStrChunk),
    Fmt(FStrChunk),
    Raw(Span),
}

pub enum PStrChunk {
    Raw(Span),
    Backslash,
    SQuote,
    DQuote,
    NewLine,
    Return,
    Tab,
}

pub enum FStrChunk {
    Raw(Span),
    Expr(Box<Expr>, Option<Fmt>),
    LBrace,
    RBrace,
    Backslash,
    SQuote,
    DQuote,
    NewLine,
    Return,
    Tab,
}

pub struct Fmt {
    pub align: Align,
    pub len: Int,
}

pub enum Align {
    Left,
    Right,
    Middle,
}
