use pegmacro::Unwrap;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum CacheType {
    Expect(&'static str),
    Integer,
}

#[derive(Debug, Clone, Unwrap)]
pub enum CacheResult {
    Expect(Option<&'static str>),
    Integer(Option<String>),
}

