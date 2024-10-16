use crate::memo::Memo;
use crate::stream::Stream;
use crate::table::Table;

mod memo;
mod table;
mod stream;

pub struct Parser<CT, CR> {
    pub memo: Memo<CT, CR>,
    pub table: Table,
    pub stream: Stream,
    pub cut: bool,
}

impl<CT, CR, S> Parser<CT, CR> {
    pub fn alter<T, F>(&mut self, f: F) -> Option<Option<T>>
    where
        F: Fn(&mut Parser<CT, CR>) -> Option<T>,
    {
        let mode = self.stream.strict;
        let pos = self.stream.cursor;

        let result = f(self);
        let cut = self.cut;

        self.cut = false;
        self.stream.strict = mode;
        if result.is_none() {
            self.stream.cursor = pos;
        }

        if cut || result.is_some() {
            Some(result)
        } else {
            None
        }
    }

    pub fn expect(&mut self, s: &'static str) -> Option<&'static str> {
        if let Some(res) = self.alter(|x| {
            x.stream.trim();
            x.stream.strict = true;
            s.chars().all(|c| x.stream.next() == Some(c)).then_some(s)
        }) {
            return res;
        }
        None
    }

    pub fn scan(&mut self, filter: fn(char) -> bool) -> Option<char> {
        let pos = self.stream.cursor;
        let saw = self.stream.next()?;
        if filter(saw) {
            Some(saw)
        } else {
            self.stream.cursor = pos;
            None
        }
    }

    pub fn lookahead(&mut self, filter: fn(char) -> bool) -> Option<char> {
        let pos = self.stream.cursor;
        let saw = self.stream.next().unwrap_or('\0');
        self.stream.cursor = pos;
        if filter(saw) {
            Some(saw)
        } else {
            None
        }
    }
}
