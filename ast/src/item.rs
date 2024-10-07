use crate::pat::{Ident, Path};
use crate::stmt::Stmt;

pub enum Item {
    /// statement, but not in a function:
    /// ```ignore
    /// 1 + 1;
    /// ```
    Stmt(Stmt),
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
