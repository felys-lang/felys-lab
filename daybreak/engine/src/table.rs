use std::collections::HashMap;
use std::hash::Hash;

pub struct Table {
    body: HashMap<String, usize>,
}

impl Table {
    pub fn id(&mut self, s: String) -> usize {
        let new = self.body.len();
        *self.body.entry(s).or_insert(new)
    }
}
