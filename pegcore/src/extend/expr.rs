use crate::ast::*;
use crate::core::Parser;
use pegmacro::memoize;

impl Parser {
    #[memoize(cache = Namespace)]
    pub fn namespace(&mut self) -> Option<Namespace> {
        let pos = self.stream.mark();
        let mut cr = CacheResult::Namespace(None);
        let mut end = pos;
        loop {
            self.cache.insert(pos, CacheType::Namespace, end, cr.clone());
            let res = self.namespace_left_rec();
            if end < self.stream.mark() {
                cr = CacheResult::Namespace(res);
                end = self.stream.mark();
                self.stream.jump(pos);
            } else {
                self.stream.jump(end);
                break cr.into();
            }
        }
    }

    fn namespace_left_rec(&mut self) -> Option<Namespace> {
        let pos = self.stream.mark();
        if let Some(result) = || -> Option<Namespace> {
            let ns = self.namespace()?;
            self.expect("::")?;
            let name = self.name()?;
            Some(Namespace::Space { ns: Box::new(ns), name })
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        if let Some(result) = || -> Option<Namespace> {
            let name = self.name()?;
            Some(Namespace::Name(name))
        }() {
            return Some(result);
        } else {
            self.stream.jump(pos)
        }
        None
    }
}
