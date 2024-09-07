use crate::ast::*;
use std::fmt::{Display, Formatter};

impl Display for Disjunction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Disjunction::Rec {
                lhs,
                rhs
            } => write!(f, "{} or {}", lhs, rhs),
            Disjunction::Plain(x) => write!(f, "{}", x)
        }
    }
}

impl Display for Conjunction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Conjunction::Rec {
                lhs,
                rhs
            } => write!(f, "{} and {}", lhs, rhs),
            Conjunction::Plain(x) => write!(f, "{}", x)
        }
    }
}

impl Display for Inversion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Inversion::Rec(x) => write!(f, "not {}", x),
            Inversion::Plain(x) => write!(f, "{}", x)
        }
    }
}

impl Display for Comparison {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Comparison::Rec {
                lhs,
                op,
                rhs
            } => write!(f, "{} {} {}", lhs, op, rhs),
            Comparison::Plain(x) => write!(f, "{}", x)
        }
    }
}

impl Display for ComOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ComOp::Gt => write!(f, ">"),
            ComOp::Ge => write!(f, ">="),
            ComOp::Lt => write!(f, "<="),
            ComOp::Le => write!(f, ">"),
            ComOp::Eq => write!(f, "=="),
            ComOp::Ne => write!(f, "!="),
        }
    }
}

impl Display for Additive {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Additive::Rec {
                lhs,
                op,
                rhs
            } => write!(f, "{} {} {}", lhs, op, rhs),
            Additive::Plain(x) => write!(f, "{}", x)
        }
    }
}

impl Display for AddOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AddOp::Add => write!(f, "+"),
            AddOp::Sub => write!(f, "-"),
        }
    }
}

impl Display for Multiplicity {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Multiplicity::Rec {
                lhs,
                op,
                rhs
            } => write!(f, "{} {} {}", lhs, op, rhs),
            Multiplicity::Plain(x) => write!(f, "{}", x)
        }
    }
}

impl Display for MulOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MulOp::Mul => write!(f, "*"),
            MulOp::Div => write!(f, "/"),
            MulOp::Mod => write!(f, "%"),
        }
    }
}

impl Display for Unary {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Unary::Rec {
                op,
                inner
            } => write!(f, "{}{}", op, inner),
            Unary::Plain(x) => write!(f, "{}", x)
        }
    }
}

impl Display for UnaOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            UnaOp::Pos => write!(f, "+"),
            UnaOp::Neg => write!(f, "-"),
        }
    }
}

impl Display for Evaluation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Evaluation::Call {
                ident
            } => write!(f, "{}()", ident),
            Evaluation::Member {
                ident,
                member
            } => write!(f, "{}.{}", ident, member),
            Evaluation::Primary(e) => write!(f, "{}", e)
        }
    }
}

impl Display for Primary {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Primary::Parentheses(p) => write!(f, "({})", p),
            Primary::Identifier(p) => write!(f, "{}", p),
            Primary::Integer(p) => write!(f, "{}", p),
            Primary::Decimal(p) => write!(f, "{}", p),
            Primary::Boolean(p) => write!(f, "{}", p),
        }
    }
}

impl Display for Namespace {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Namespace::Space {
                ns,
                name
            } => write!(f, "{}::{}", ns, name),
            Namespace::Name(n) => write!(f, "{}", n)
        }
    }
}

impl Display for Integer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Integer::Base16(i) => write!(f, "0x{}", i),
            Integer::Base10(i) => write!(f, "{}", i),
            Integer::Base8(i) => write!(f, "0o{}", i),
            Integer::Base2(i) => write!(f, "0b{}", i),
        }
    }
}

impl Display for Decimal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}", self.whole, self.frac)
    }
}

impl Display for Boolean {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Boolean::True => write!(f, "true"),
            Boolean::False => write!(f, "false")
        }
    }
}
