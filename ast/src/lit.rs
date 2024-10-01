use crate::expr::Expr;
use crate::Span;

pub enum Lit {
    Int(Int),
    Deci(Deci),
    Bool(Bool),
    Str(Str)
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
    SingleQuote,
    DoubleQuote,
    NewLine,
    Return,
    Tab,
}

pub enum FStrChunk {
    Raw(Span),
    Expr(Box<Expr>, Option<Fmt>),
    OpenBrace,
    CloseBrace,
    Backslash,
    SingleQuote,
    DoubleQuote,
    NewLine,
    Return,
    Tab,
}

pub struct Fmt {
    pub align: Align,
    pub len: Int
}

pub enum Align {
    Left,
    Right,
    Middle,
}
