use crate::shared::ast::*;
use std::fmt::{Display, Formatter};

impl Display for ElyDisjunction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ElyDisjunction::Rec {
                lhs,
                rhs
            } => write!(f, "{} or {}", lhs, rhs),
            ElyDisjunction::Plain(x) => write!(f, "{}", x)
        }
    }
}

impl Display for ElyConjunction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ElyConjunction::Rec {
                lhs,
                rhs
            } => write!(f, "{} and {}", lhs, rhs),
            ElyConjunction::Plain(x) => write!(f, "{}", x)
        }
    }
}

impl Display for ElyInversion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ElyInversion::Rec(x) => write!(f, "not {}", x),
            ElyInversion::Plain(x) => write!(f, "{}", x)
        }
    }
}

impl Display for ElyComparison {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ElyComparison::Rec {
                lhs,
                op,
                rhs
            } => write!(f, "{} {} {}", lhs, op, rhs),
            ElyComparison::Plain(x) => write!(f, "{}", x)
        }
    }
}

impl Display for ElyComOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ElyComOp::Gt => write!(f, ">"),
            ElyComOp::Ge => write!(f, ">="),
            ElyComOp::Lt => write!(f, "<="),
            ElyComOp::Le => write!(f, ">"),
            ElyComOp::Eq => write!(f, "=="),
            ElyComOp::Ne => write!(f, "!="),
        }
    }
}

impl Display for ElyAdditive {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ElyAdditive::Rec {
                lhs,
                op,
                rhs
            } => write!(f, "{} {} {}", lhs, op, rhs),
            ElyAdditive::Plain(x) => write!(f, "{}", x)
        }
    }
}

impl Display for ElyAddOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ElyAddOp::Add => write!(f, "+"),
            ElyAddOp::Sub => write!(f, "-"),
        }
    }
}

impl Display for ElyMultiplicity {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ElyMultiplicity::Rec {
                lhs,
                op,
                rhs
            } => write!(f, "{} {} {}", lhs, op, rhs),
            ElyMultiplicity::Plain(x) => write!(f, "{}", x)
        }
    }
}

impl Display for ElyMulOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ElyMulOp::Mul => write!(f, "*"),
            ElyMulOp::Div => write!(f, "/"),
            ElyMulOp::Mod => write!(f, "%"),
        }
    }
}

impl Display for ElyUnary {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ElyUnary::Rec {
                op,
                inner
            } => write!(f, "{}{}", op, inner),
            ElyUnary::Plain(x) => write!(f, "{}", x)
        }
    }
}

impl Display for ElyUnaOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ElyUnaOp::Pos => write!(f, "+"),
            ElyUnaOp::Neg => write!(f, "-"),
        }
    }
}

impl Display for ElyEvaluation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ElyEvaluation::Call {
                ident,
                args
            } => {
                let args = args.iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(", ");
                write!(f, "{}({})", ident, args)
            }
            ElyEvaluation::Member {
                ident,
                member
            } => write!(f, "{}.{}", ident, member),
            ElyEvaluation::Primary(e) => write!(f, "{}", e)
        }
    }
}

impl Display for ElyPrimary {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ElyPrimary::Parentheses(x) => write!(f, "({})", x),
            ElyPrimary::Identifier(x) => write!(f, "{}", x),
            ElyPrimary::Integer(x) => write!(f, "{}", x),
            ElyPrimary::Decimal(x) => write!(f, "{}", x),
            ElyPrimary::Boolean(x) => write!(f, "{}", x),
            ElyPrimary::String(x) => write!(f, "{}", x)
        }
    }
}

impl Display for ElyNamespace {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ElyNamespace::Space {
                ns,
                name
            } => write!(f, "{}::{}", ns, name),
            ElyNamespace::Name(n) => write!(f, "{}", n)
        }
    }
}

impl Display for ElyInteger {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ElyInteger::Base16(i) => write!(f, "0x{}", i),
            ElyInteger::Base10(i) => write!(f, "{}", i),
            ElyInteger::Base8(i) => write!(f, "0o{}", i),
            ElyInteger::Base2(i) => write!(f, "0b{}", i),
        }
    }
}

impl Display for ElyDecimal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}", self.whole, self.frac)
    }
}

impl Display for ElyBoolean {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ElyBoolean::True => write!(f, "true"),
            ElyBoolean::False => write!(f, "false")
        }
    }
}

impl Display for ElyString {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ElyString::Format(v) => {
                let string = v.iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join("");
                write!(f, "f\"{}\"", string)
            }
            ElyString::Plain(v) => {
                let string = v.iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join("");
                write!(f, "\"{}\"", string)
            }
            ElyString::Raw(x) => write!(f, "r\"{}\"", x)
        }
    }
}

impl Display for ElyFmtChar {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ElyFmtChar::Placeholder(x) => write!(f, "{{ {} }}", x),
            ElyFmtChar::Plain(x) => write!(f, "{}", x),
            ElyFmtChar::Close => write!(f, "}}"),
            ElyFmtChar::Open => write!(f, "{{"),
        }
    }
}

impl Display for ElyPlaceHolder {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} : {} {}", self.expr, self.align, self.len)
    }
}

impl Display for ElyAlign {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ElyAlign::Left => write!(f, "<"),
            ElyAlign::Right => write!(f, ">"),
            ElyAlign::Middle => write!(f, "^"),
        }
    }
}

impl Display for ElyChar {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ElyChar::Plain(x) => write!(f, "{}", x),
            ElyChar::Return => write!(f, "\\r"),
            ElyChar::Quotation => write!(f, "\\\""),
            ElyChar::Backslash => write!(f, "\\\\"),
            ElyChar::NewLine => write!(f, "\\n"),
            ElyChar::Tab => write!(f, "\\t"),
        }
    }
}