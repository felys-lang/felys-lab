use crate::ID;

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
    Base16(ID),
    Base10(ID),
    Base8(ID),
    Base2(ID),
}

pub enum Deci {
    Finite(ID),
    Inf,
}

pub enum Bool {
    True,
    False,
}

pub enum Str {
    Plain(ID),
    Format(ID),
    Raw(ID),
}
