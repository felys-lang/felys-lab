pub enum Pat {
    Wild,
    Ident(Ident)
}

pub struct Ident {
    pub id: usize
}
