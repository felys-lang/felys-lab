use std::collections::HashMap;
use std::hash::Hash;

pub struct Memo<T, R> {
    pub(crate) body: HashMap<(usize, bool, T), (usize, Option<R>)>,
}

impl<T: Eq + Hash, R: Clone> Memo<T, R> {
    pub fn recall(&self, p: usize, s: bool, ct: T) -> Option<(usize, Option<R>)> {
        self.body.get(&(p, s, ct)).cloned()
    }

    pub fn cache(&mut self, p: usize, s: bool, ct: T, e: usize, cr: Option<R>) {
        self.body.insert((p, s, ct), (e, cr));
    }
}
