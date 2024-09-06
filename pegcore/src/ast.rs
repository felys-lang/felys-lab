use pegmacro::CRInner;

pub struct Program(pub Primary);

#[derive(Debug, Clone)]
pub struct Expression;

#[derive(Debug, Clone)]
pub enum Primary {
    Evaluation(Evaluation),
    Integer(Integer),
    Decimal(Decimal),
    Boolean(Boolean),
}

#[derive(Debug, Clone)]
pub enum Evaluation {
    Call {
        ident: Box<Evaluation>,
    },
    Member {
        ident: Box<Evaluation>,
        member: Name,
    },
    Namespace(Namespace)
}

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
    Evaluation,
    Namespace,
    Primary,
    Boolean,
    Decimal,
    Integer,
    Name,
}

#[derive(Debug, Clone, CRInner)]
pub enum CacheResult {
    Expect(Option<&'static str>),
    Evaluation(Option<Evaluation>),
    Namespace(Option<Namespace>),
    Primary(Option<Primary>),
    Boolean(Option<Boolean>),
    Decimal(Option<Decimal>),
    Integer(Option<Integer>),
    Name(Option<Name>),
}
