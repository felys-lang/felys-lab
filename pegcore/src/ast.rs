#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum CacheType {
    Expect(&'static char),
}

#[derive(Debug, Clone)]
pub enum CacheResult {
    Expect(Option<&'static char>),
}
