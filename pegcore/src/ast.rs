use pegmacro::Unwrap;

pub struct Program(pub Integer);

pub type Name = String;

#[derive(Debug, Clone)]
pub enum Integer {
    Base16(String),
    Base10(String),
    Base8(String),
    Base2(String),
}

#[derive(Debug, Clone)]
pub struct Decimal {
    pub whole: String,
    pub frac: String,
}

#[derive(Debug, Clone)]
pub enum Boolean {
    True,
    False,
}


#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum CacheType {
    Expect(&'static str),
    Boolean,
    Decimal,
    Integer,
    Name,
}

#[derive(Debug, Clone, Unwrap)]
pub enum CacheResult {
    Expect(Option<&'static str>),
    Boolean(Option<Boolean>),
    Decimal(Option<Decimal>),
    Integer(Option<Integer>),
    Name(Option<Name>),
}
