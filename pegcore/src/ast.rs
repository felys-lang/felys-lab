use pegmacro::CRInner;

pub type Program = Additive;

#[derive(Debug, Clone)]
pub enum Additive {
    Additive {
        lhs: Box<Additive>,
        op: AddOp,
        rhs: Multiplicity,
    },
    Multiplicity(Multiplicity),
}

#[derive(Debug, Clone)]
pub enum AddOp {
    Add,
    Sub,
}


#[derive(Debug, Clone)]
pub enum Multiplicity {
    Multiplicity {
        lhs: Box<Multiplicity>,
        op: MulOp,
        rhs: Unary,
    },
    Unary(Unary),
}

#[derive(Debug, Clone)]
pub enum MulOp {
    Mul,
    Div,
    Mod,
}

#[derive(Debug, Clone)]
pub enum Unary {
    Unary {
        op: UnaOp,
        inner: Box<Unary>,
    },
    Primary(Primary),
}

#[derive(Debug, Clone)]
pub enum UnaOp {
    Pos,
    Neg,
}

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
    Namespace(Namespace),
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
    Multiplicity,
    Evaluation,
    Namespace,
    Additive,
    Primary,
    Boolean,
    Decimal,
    Integer,
    Unary,
    Name,
}

#[derive(Debug, Clone, CRInner)]
pub enum CacheResult {
    Expect(Option<&'static str>),
    Multiplicity(Option<Multiplicity>),
    Evaluation(Option<Evaluation>),
    Namespace(Option<Namespace>),
    Additive(Option<Additive>),
    Primary(Option<Primary>),
    Boolean(Option<Boolean>),
    Decimal(Option<Decimal>),
    Integer(Option<Integer>),
    Unary(Option<Unary>),
    Name(Option<Name>),
}
