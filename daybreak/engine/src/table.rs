use std::collections::HashMap;
use std::hash::Hash;
use std::rc::Rc;

pub struct Table {
    pub(crate) body: HashMap<Rc<String>, usize>,
    pub(crate) fast: Vec<Rc<String>>,
}

impl Table {
    pub fn id(&mut self, s: String) -> usize {
        if let Some(&id) = self.body.get(&s) {
            id
        } else {
            let key = Rc::new(s);
            let val = self.fast.len();
            self.fast.push(key.clone());
            self.body.insert(key, val);
            val
        }
    }

    pub fn get(&self, id: usize) -> Option<Rc<String>> {
        self.fast.get(id).cloned()
    }
}
