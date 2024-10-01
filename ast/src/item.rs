use crate::lit::Int;
use crate::pat::{Ident, Path};
use crate::stmt::Stmt;

pub enum Item {
    /// enumeration that carries represent an integer:
    /// ```ignore
    /// enum Firemoth {
    ///     Elysia,
    ///     Pardofelis,
    /// }
    /// ```
    Enum(Ident, EnumDef),
    /// statement, but not in a function:
    /// ```ignore
    /// 1 + 1;
    /// ```
    Stmt(Stmt),
    /// struct type:
    /// ```ignore
    /// struct Elysia {
    ///     name: str,
    /// }
    /// ```
    Struct(Ident, Vec<Path>, StructDef),
    /// union type:
    /// ```ignore
    /// enum Elysia {
    ///     HumanEgo(str),
    ///     MissPink {
    ///         name: str,
    ///     },
    /// }
    /// ```
    Union(Ident, Vec<Path>, Vec<Variant>),
    /// import things from other files:
    /// ```ignore
    /// use firemoth::{Elysia as Ely, Pardofelis};
    /// ```
    Use(UseTree),
}

pub struct UseTree {
    pub prefix: Path,
    pub leaf: UseLeaf,
}

pub enum UseLeaf {
    More(Vec<UseTree>),
    Alias(Option<Ident>),
    All,
}

pub enum EnumDef {
    Auto(Vec<Ident>),
    Explicit(Vec<EnumValue>),
}

pub struct EnumValue {
    pub name: Ident,
    pub value: Int,
}

pub enum StructDef {
    Struct(Vec<Field>),
    Tuple(Vec<Path>),
}

pub struct Field {
    pub name: Ident,
    pub kind: Path,
}

pub enum Variant {
    Struct(Ident, Vec<Field>),
    Tuple(Ident, Vec<Path>),
    Simple(Ident),
}
