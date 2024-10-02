use crate::lit::Lit;
use crate::Span;

pub enum Pat {
    /// don't care: `_`
    Any,
    /// unwrap a group: `(elysia, 11.11)`
    Group(Vec<Pat>),
    /// literals: `"elysia"`, `11.11`, `true`
    Lit(Lit),
    /// or: `'0' | '1'`
    Or(Vec<Pat>),
    /// path, aka namespace: `firemoth::elysia`
    Path(Path),
    /// range: `0..=41`
    Range(Option<Lit>, Option<Lit>),
    /// placeholder for the rest: `Elysia { alive, .. }`
    Rest,
    /// unwrap a struct: `Elysia { alive }`
    Struct(Path, Vec<PatField>),
    /// unwrap a tuple struct: `Elysia(alive)`
    TupStruct(Path, Vec<Pat>),
}

pub struct PatField {
    pub name: Ident,
    pub pat: Pat,
}

pub type Path = Vec<PathSeg>;

pub struct PathSeg {
    pub name: Ident,
    pub arg: Vec<Path>,
}

pub type Ident = Span;
