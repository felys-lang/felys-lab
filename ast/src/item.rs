use crate::lit::Int;
use crate::pat::{Ident, Path};
use crate::stmt::Stmt;

pub enum Item {
    /// enumeration that carries represent an integer:
    /// ```
    /// enum Firemoth {
    ///     Elysia,
    ///     Kevin,
    /// }
    /// ```
    Enum(Ident, Vec<EnumVal>),
    /// statement, but not in a function:
    /// ```
    /// 1 + 1;
    /// ```
    Stmt(Stmt),
    /// struct type:
    /// ```
    /// struct Elysia {
    ///     alive: bool,
    /// }
    /// ```
    Struct(Ident, Vec<Path>, Vec<FieldDef>),
    /// union type:
    /// ```
    /// enum Elysia {
    ///     HumanEgo(bool),
    ///     MissPink(bool),
    /// }
    /// ```
    Union(Ident, Vec<Path>, Vec<Variant>),
    /// import things from other files:
    /// ```
    /// use std::collections::{HashMap as Map, HashSet as Set};
    /// ```
    Use(UseTree),
}

pub struct UseTree {
    pub prefix: Path,
    pub leaf: UseLeaf
}

pub enum UseLeaf {
    More(Vec<UseTree>),
    Alias(Option<Ident>),
    All
}

pub struct FieldDef {
    pub name: Ident,
    pub kind: Path,
}

pub struct EnumVal {
    pub name: Ident,
    pub value: Option<Int>,
}

pub enum Variant {
    Struct(Ident, Vec<FieldDef>),
    Tuple(Ident, Vec<Path>),
    Simple(Ident),
}
