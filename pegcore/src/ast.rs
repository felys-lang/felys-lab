#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum CacheType {
    Expect(&'static str),
    Integer,
}

#[derive(Debug, Clone)]
pub enum CacheResult {
    Expect(Option<&'static str>),
    Integer(Option<String>),
}


impl From<CacheResult> for Option<&'static str> {
    fn from(value: CacheResult) -> Self {
        match value {
            CacheResult::Expect(inner) => inner,
            _ => panic!("cache unmatched")
        }
    }
}

impl From<CacheResult> for Option<String> {
    fn from(value: CacheResult) -> Self {
        match value {
            CacheResult::Integer(inner) => inner,
            _ => panic!("cache unmatched")
        }
    }
}