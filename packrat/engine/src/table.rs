use std::collections::HashMap;
use std::hash::Hash;
use utils::{Symbol, P};

pub struct Table {
    pub(crate) body: HashMap<P<String>, Symbol>,
    pub(crate) fast: Vec<P<String>>,
}

impl Table {
    pub fn id(&mut self, s: String) -> Symbol {
        if let Some(&id) = self.body.get(&s) {
            id
        } else {
            let key = P::new(s);
            let val = self.fast.len();
            self.fast.push(key.clone());
            self.body.insert(key, val);
            val
        }
    }

    pub fn get(&self, id: usize) -> Option<P<String>> {
        self.fast.get(id).cloned()
    }
}
