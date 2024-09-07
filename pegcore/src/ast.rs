use pegmacro::CR;

pub type Program = Expression;

pub type Expression = Disjunction;

#[derive(Debug, Clone)]
pub enum Disjunction {
    Rec {
        lhs: Box<Disjunction>,
        rhs: Conjunction
    },
    Plain(Conjunction)
}

#[derive(Debug, Clone)]
pub enum Conjunction {
    Rec {
        lhs: Box<Conjunction>,
        rhs: Inversion
    },
    Plain(Inversion)
}

#[derive(Debug, Clone)]
pub enum Inversion {
    Rec(Box<Inversion>),
    Plain(Comparison)
}

#[derive(Debug, Clone)]
pub enum Comparison {
    Rec {
        lhs: Box<Comparison>,
        op: ComOp,
        rhs: Additive,
    },
    Plain(Additive),
}

#[derive(Debug, Clone)]
pub enum ComOp {
    Gt,
    Ge,
    Lt,
    Le,
    Eq,
    Ne,
}

#[derive(Debug, Clone)]
pub enum Additive {
    Rec {
        lhs: Box<Additive>,
        op: AddOp,
        rhs: Multiplicity,
    },
    Plain(Multiplicity),
}

#[derive(Debug, Clone)]
pub enum AddOp {
    Add,
    Sub,
}

#[derive(Debug, Clone)]
pub enum Multiplicity {
    Rec {
        lhs: Box<Multiplicity>,
        op: MulOp,
        rhs: Unary,
    },
    Plain(Unary),
}

#[derive(Debug, Clone)]
pub enum MulOp {
    Mul,
    Div,
    Mod,
}

#[derive(Debug, Clone)]
pub enum Unary {
    Rec {
        op: UnaOp,
        inner: Box<Unary>,
    },
    Plain(Evaluation),
}

#[derive(Debug, Clone)]
pub enum UnaOp {
    Pos,
    Neg,
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
    Primary(Primary),
}

#[derive(Debug, Clone)]
pub enum Primary {
    Parentheses(Box<Expression>),
    Identifier(Namespace),
    Integer(Integer),
    Decimal(Decimal),
    Boolean(Boolean),
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
    Disjunction,
    Conjunction,
    Inversion,
    Comparison,
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

#[derive(Debug, Clone, CR)]
pub enum CacheResult {
    Expect(Option<&'static str>),
    Multiplicity(Option<Multiplicity>),
    Comparison(Option<Comparison>),
    Evaluation(Option<Evaluation>),
    Disjunction(Option<Disjunction>),
    Conjunction(Option<Conjunction>),
    Inversion(Option<Inversion>),
    Namespace(Option<Namespace>),
    Additive(Option<Additive>),
    Primary(Option<Primary>),
    Boolean(Option<Boolean>),
    Decimal(Option<Decimal>),
    Integer(Option<Integer>),
    Unary(Option<Unary>),
    Name(Option<Name>),
}
