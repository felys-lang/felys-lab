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
    Enum(Ident, EnumDef),
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
    Struct(Ident, Vec<Path>, StructDef),
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
    Named(Vec<Field>),
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
