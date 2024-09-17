pub type ElyExpression = ElyDisjunction;

#[daybreak::ast]
pub enum ElyDisjunction {
    Rec {
        lhs: Box<ElyDisjunction>,
        rhs: ElyConjunction,
    },
    Plain(ElyConjunction),
}

#[daybreak::ast]
pub enum ElyConjunction {
    Rec {
        lhs: Box<ElyConjunction>,
        rhs: ElyInversion,
    },
    Plain(ElyInversion),
}

#[daybreak::ast]
pub enum ElyInversion {
    Rec(Box<ElyInversion>),
    Plain(ElyComparison),
}

#[daybreak::ast]
pub enum ElyComparison {
    Rec {
        lhs: Box<ElyComparison>,
        op: ElyComOp,
        rhs: ElyAdditive,
    },
    Plain(ElyAdditive),
}

#[daybreak::ast]
pub enum ElyComOp {
    Gt,
    Ge,
    Lt,
    Le,
    Eq,
    Ne,
}

#[daybreak::ast]
pub enum ElyAdditive {
    Rec {
        lhs: Box<ElyAdditive>,
        op: ElyAddOp,
        rhs: ElyMultiplicity,
    },
    Plain(ElyMultiplicity),
}

#[daybreak::ast]
pub enum ElyAddOp {
    Add,
    Sub,
}

#[daybreak::ast]
pub enum ElyMultiplicity {
    Rec {
        lhs: Box<ElyMultiplicity>,
        op: ElyMulOp,
        rhs: ElyUnary,
    },
    Plain(ElyUnary),
}

#[daybreak::ast]
pub enum ElyMulOp {
    Mul,
    Div,
    Mod,
}

#[daybreak::ast]
pub enum ElyUnary {
    Rec {
        op: ElyUnaOp,
        inner: Box<ElyUnary>,
    },
    Plain(ElyEvaluation),
}

#[daybreak::ast]
pub enum ElyUnaOp {
    Pos,
    Neg,
}

pub type ElyArguments = Vec<ElyExpression>;

#[daybreak::ast]
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

#[daybreak::ast]
pub enum ElyPrimary {
    Parentheses(Box<ElyExpression>),
    Tuple(Vec<ElyExpression>),
    Identifier(ElyNamespace),
    Integer(ElyInteger),
    Decimal(ElyDecimal),
    Boolean(ElyBoolean),
    String(ElyString),
}

#[daybreak::ast]
pub enum ElyNamespace {
    Space {
        ns: Box<ElyNamespace>,
        name: ElyName,
    },
    Name(ElyName),
}

pub type ElyName = String;

#[daybreak::ast]
pub enum ElyInteger {
    Base16(String),
    Base10(String),
    Base8(String),
    Base2(String),
}

#[daybreak::ast]
pub struct ElyDecimal {
    pub whole: String,
    pub frac: String,
}

#[daybreak::ast]
pub enum ElyBoolean {
    True,
    False,
}

#[daybreak::ast]
pub enum ElyString {
    Format(Vec<ElyFmtChar>),
    Plain(Vec<ElyChar>),
    Raw(String),
}

#[daybreak::ast]
pub enum ElyFmtChar {
    Placeholder {
        expr: Option<ElyExpression>,
        fmt: Option<ElyFormatter>,
    },
    Plain(ElyChar),
    Close,
    Open,
}

#[daybreak::ast]
pub struct ElyFormatter {
    pub debug: bool,
    pub align: Option<ElyAlign>,
    pub len: Option<ElyInteger>,
}

#[daybreak::ast]
pub enum ElyAlign {
    Left,
    Right,
    Middle,
}

#[daybreak::ast]
pub enum ElyChar {
    Plain(char),
    Backslash,
    Quotation,
    NewLine,
    Return,
    Tab,
}
