use std::collections::HashMap;
use std::hash::Hash;

pub struct Table<S> {
    body: HashMap<S, usize>,
}

impl<S: Eq + Hash> Table<S> {
    pub fn id(&mut self, s: S) -> usize {
        let new = self.body.len();
        *self.body.entry(s).or_insert(new)
    }
}
