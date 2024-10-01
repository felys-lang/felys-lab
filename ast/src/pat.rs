use crate::lit::Lit;
use crate::Span;

pub enum Pat {
    /// group of things: `(elysia, 11.11)`
    Group(Vec<Pat>),
    /// literals: `"elysia"`, `11.11`, `true`
    Lit(Lit),
    /// path, aka namespace: `firemoth::elysia`
    Path(Path),
    /// don't care: `_`
    Underscore,
}

pub type Path = Vec<PathSeg>;

pub struct PathSeg {
    pub name: Ident,
    pub arg: Vec<Path>,
}

pub type Ident = Span;
