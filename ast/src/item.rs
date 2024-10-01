use crate::stmt::Stmt;

pub enum Item {
    Stmt(Stmt),
    Use,
    Trait,
    Impl,
    Type,
    Struct,
    Union,
    Enum,
}