use crate::ast::*;
use pegmacro::memoize;
use std::collections::HashMap;

pub struct Parser {
    pub stream: Stream,
    pub cache: Cache,
}

impl Parser {
    pub fn new(code: String, v: Verbose) -> Self {
        Self {
            stream: Stream {
                body: code,
                cursor: 0,
                raw: false
            },
            cache: Cache {
                body: HashMap::new(),
                verbose: v,
                hit: 0,
            },
        }
    }

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

    pub fn lookahead(&mut self, filter: fn(char) -> bool) -> Option<char> {
        let saw = self.stream.body.chars()
            .nth(self.stream.cursor)
            .unwrap_or('\u{0}');
        if filter(saw) {
            Some(saw)
        } else {
            None
        }
    }
}

pub struct Stream {
    pub body: String,
    pub cursor: usize,
    pub raw: bool
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
        let it = self.body.chars().skip(self.cursor);
        for ch in it {
            self.cursor += 1;
            if self.raw || !ch.is_whitespace() {
                return Some(ch);
            }
        }
        None
    }
}

pub struct Cache {
    pub body: HashMap<(usize, CacheType), (usize, CacheResult)>,
    pub verbose: Verbose,
    pub hit: usize,
}

#[allow(dead_code)]
#[derive(PartialOrd, PartialEq)]
pub enum Verbose {
    None,
    Core,
    Full,
}

impl Cache {
    pub fn get(&mut self, pos: usize, ct: CacheType) -> Option<(usize, CacheResult)> {
        if let Some(res) = self.body.get(&(pos, ct)) {
            if self.verbose >= Verbose::Core {
                let (end, cr) = res;
                println!("> hit\t\t{:<11} {:<23} {:<11} {}", pos, format!("{:?}", ct), end, cr)
            }
            self.hit += 1;
            Some(res.clone())
        } else {
            if self.verbose >= Verbose::Full {
                println!("> dne\t\t{:<11} {:?}", pos, ct);
            }
            None
        }
    }

    pub fn insert(&mut self, pos: usize, ct: CacheType, end: usize, cr: CacheResult) {
        if self.verbose >= Verbose::Core {
            println!("> cache\t\t{:<11} {:<23} {:<11} {}", pos, format!("{:?}", ct), end, cr)
        }
        if let Some(cache) = self.body.insert((pos, ct), (end, cr)) {
            let (end, cr) = cache;
            if self.verbose >= Verbose::Core {
                println!("> drop\t\t{:<11} {:<23} {:<11} {}", pos, format!("{:?}", ct), end, cr)
            }
        }
    }
}
