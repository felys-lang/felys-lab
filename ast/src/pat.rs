use crate::lit::Lit;
use crate::ID;

pub enum Pat {
    /// don't care: `_`
    Any,
    /// unwrap a group: `(elysia, 11.11)`
    Group(Vec<Pat>),
    /// literals: `"elysia"`, `11.11`, `true`
    Lit(Lit),
    /// path, aka namespace: `firemoth::elysia`
    Path(Path),
}

pub type Path = Vec<PathSeg>;

pub struct PathSeg {
    pub name: Ident,
    pub arg: Vec<Path>,
}

pub type Ident = ID;
