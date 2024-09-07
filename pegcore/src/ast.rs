use pegmacro::CR;

pub type ElyProgram = ElyExpression;

pub type ElyExpression = ElyDisjunction;

#[derive(Debug, Clone)]
pub enum ElyDisjunction {
    Rec {
        lhs: Box<ElyDisjunction>,
        rhs: ElyConjunction,
    },
    Plain(ElyConjunction),
}

#[derive(Debug, Clone)]
pub enum ElyConjunction {
    Rec {
        lhs: Box<ElyConjunction>,
        rhs: ElyInversion,
    },
    Plain(ElyInversion),
}

#[derive(Debug, Clone)]
pub enum ElyInversion {
    Rec(Box<ElyInversion>),
    Plain(ElyComparison),
}

#[derive(Debug, Clone)]
pub enum ElyComparison {
    Rec {
        lhs: Box<ElyComparison>,
        op: ElyComOp,
        rhs: ElyAdditive,
    },
    Plain(ElyAdditive),
}

#[derive(Debug, Clone)]
pub enum ElyComOp {
    Gt,
    Ge,
    Lt,
    Le,
    Eq,
    Ne,
}

#[derive(Debug, Clone)]
pub enum ElyAdditive {
    Rec {
        lhs: Box<ElyAdditive>,
        op: ElyAddOp,
        rhs: ElyMultiplicity,
    },
    Plain(ElyMultiplicity),
}

#[derive(Debug, Clone)]
pub enum ElyAddOp {
    Add,
    Sub,
}

#[derive(Debug, Clone)]
pub enum ElyMultiplicity {
    Rec {
        lhs: Box<ElyMultiplicity>,
        op: ElyMulOp,
        rhs: ElyUnary,
    },
    Plain(ElyUnary),
}

#[derive(Debug, Clone)]
pub enum ElyMulOp {
    Mul,
    Div,
    Mod,
}

#[derive(Debug, Clone)]
pub enum ElyUnary {
    Rec {
        op: ElyUnaOp,
        inner: Box<ElyUnary>,
    },
    Plain(ElyEvaluation),
}

#[derive(Debug, Clone)]
pub enum ElyUnaOp {
    Pos,
    Neg,
}

pub type ElyArguments = Vec<ElyExpression>;

#[derive(Debug, Clone)]
pub enum ElyEvaluation {
    Call {
        ident: Box<ElyEvaluation>,
        args: ElyArguments,
    },
    Member {
        ident: Box<ElyEvaluation>,
        member: ElyName,
    },
    Primary(ElyPrimary),
}

#[derive(Debug, Clone)]
pub enum ElyPrimary {
    Parentheses(Box<ElyExpression>),
    Identifier(ElyNamespace),
    Integer(ElyInteger),
    Decimal(ElyDecimal),
    Boolean(ElyBoolean),
    String(ElyString),
}

#[derive(Debug, Clone)]
pub enum ElyNamespace {
    Space {
        ns: Box<ElyNamespace>,
        name: ElyName,
    },
    Name(ElyName),
}

pub type ElyName = String;

#[derive(Debug, Clone)]
pub enum ElyInteger {
    Base16(String),
    Base10(String),
    Base8(String),
    Base2(String),
}

#[derive(Debug, Clone)]
pub struct ElyDecimal {
    pub whole: String,
    pub frac: String,
}

#[derive(Debug, Clone)]
pub enum ElyBoolean {
    True,
    False,
}

#[derive(Debug, Clone)]
pub enum ElyString {
    Format(Vec<ElyFmtChar>),
    Plain(Vec<ElyChar>),
    Raw(String),
}

#[derive(Debug, Clone)]
pub enum ElyFmtChar {
    Placeholder(ElyExpression),
    Plain(ElyChar),
    Close,
    Open,
}

#[derive(Debug, Clone)]
pub enum ElyChar {
    Plain(char),
    Backslash,
    Quotation,
    NewLine,
    Return,
    Tab,
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum ElyCacheType {
    ElyExpect(&'static str),
    ElyMultiplicity,
    ElyDisjunction,
    ElyConjunction,
    ElyInversion,
    ElyComparison,
    ElyEvaluation,
    ElyNamespace,
    ElyAdditive,
    ElyPrimary,
    ElyBoolean,
    ElyDecimal,
    ElyInteger,
    ElyString,
    ElyUnary,
    ElyName,
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, CR)]
pub enum ElyCacheResult {
    ElyExpect(Option<&'static str>),
    ElyMultiplicity(Option<ElyMultiplicity>),
    ElyComparison(Option<ElyComparison>),
    ElyEvaluation(Option<ElyEvaluation>),
    ElyDisjunction(Option<ElyDisjunction>),
    ElyConjunction(Option<ElyConjunction>),
    ElyInversion(Option<ElyInversion>),
    ElyNamespace(Option<ElyNamespace>),
    ElyAdditive(Option<ElyAdditive>),
    ElyPrimary(Option<ElyPrimary>),
    ElyBoolean(Option<ElyBoolean>),
    ElyDecimal(Option<ElyDecimal>),
    ElyInteger(Option<ElyInteger>),
    ElyString(Option<ElyString>),
    ElyUnary(Option<ElyUnary>),
    ElyName(Option<ElyName>),
}
