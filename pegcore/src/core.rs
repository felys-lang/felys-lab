use crate::ast::*;
use pegmacro::memoize;
use std::collections::HashMap;

pub struct Parser {
    pub stream: Stream,
    pub cache: Cache,
}

#[allow(dead_code)]
impl Parser {
    #[memoize(cache = Expect)]
    pub fn expect(&mut self, s: &'static str) -> Option<&'static str> {
        let pos = self.stream.mark();
        for ch in s.chars() {
            let sn = self.stream.next();
            if !matches!(sn, Some(d) if d == ch) {
                self.stream.jump(pos);
                return None;
            }
        }
        Some(s)
    }

    pub fn scan(&mut self, filter: fn(char) -> bool) -> Option<char> {
        let pos = self.stream.mark();
        let saw = self.stream.next()?;
        if filter(saw) {
            Some(saw)
        } else {
            self.stream.jump(pos);
            None
        }
    }

    pub fn pl(&mut self, ch: char) -> Option<char> {
        let pos = self.stream.mark();
        let saw = self.stream.next()?;
        self.stream.jump(pos);
        if ch == saw {
            Some(saw)
        } else {
            None
        }
    }

    pub fn nl(&mut self, ch: char) -> Option<char> {
        let pos = self.stream.mark();
        let saw = self.stream.next()?;
        self.stream.jump(pos);
        if ch != saw {
            Some(saw)
        } else {
            None
        }
    }
}

pub struct Stream {
    pub body: String,
    pub cursor: usize,
}

impl Stream {
    pub fn mark(&self) -> usize {
        self.cursor
    }

    pub fn jump(&mut self, pos: usize) {
        self.cursor = pos
    }
}

impl Iterator for Stream {
    type Item = char;
    fn next(&mut self) -> Option<Self::Item> {
        let ch = self.body.chars().nth(self.cursor)?;
        self.cursor += 1;
        Some(ch)
    }
}

pub struct Cache {
    pub body: HashMap<(usize, CacheType), (usize, CacheResult)>,
    pub verbose: bool,
    pub hit: usize,
}

impl Cache {
    pub fn get(&mut self, pos: usize, ct: CacheType) -> Option<(usize, CacheResult)> {
        if let Some(res) = self.body.get(&(pos, ct)) {
            if self.verbose {
                let (end, cr) = res;
                println!("{}\t{}\t{:?} => {:?}", pos, end, ct, cr)
            }
            self.hit += 1;
            Some(res.clone())
        } else {
            None
        }
    }

    pub fn insert(&mut self, pos: usize, ct: CacheType, end: usize, cr: CacheResult) {
        if self.verbose {
            println!("{}\t{}\t{:?} => {:?}", pos, end, ct, cr)
        }
        if self.body.insert((pos, ct), (end, cr)).is_some() {
            panic!("cache conflicted")
        }
    }
}
