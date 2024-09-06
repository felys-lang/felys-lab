use pegmacro::CRInner;

pub struct Program(pub Namespace);

#[derive(Debug, Clone)]
pub enum Namespace {
    Space {
        ns: Box<Namespace>,
        name: Name,
    },
    Name(Name),
}

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
    Namespace,
    Boolean,
    Decimal,
    Integer,
    Name,
}

#[derive(Debug, Clone, CRInner)]
pub enum CacheResult {
    Expect(Option<&'static str>),
    Namespace(Option<Namespace>),
    Boolean(Option<Boolean>),
    Decimal(Option<Decimal>),
    Integer(Option<Integer>),
    Name(Option<Name>),
}
