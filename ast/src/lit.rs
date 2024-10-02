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

pub enum Deci {
    Finite(Span),
    Inf,
}

pub enum Bool {
    True,
    False,
}

pub enum Str {
    Plain(Span),
    Format(Span),
    Raw(Span),
}
