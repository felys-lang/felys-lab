pub mod expr;
pub mod lit;
pub mod stmt;
pub mod pat;
pub mod item;

pub type Span = (usize, usize);


#[cfg(test)]
#[cfg(target_pointer_width = "64")]
mod test {
    use crate::*;

    macro_rules! assert_size_eq {
        ($t:ty, $s:expr) => {
            assert_eq!(size_of::<$t>(), $s);
        };
    }

    #[test]
    fn size() {
        assert_size_eq!(expr::Expr, 56);
        assert_size_eq!(pat::Pat, 72);
        assert_size_eq!(lit::Lit, 32);
        assert_size_eq!(item::Item, 72);
        assert_size_eq!(stmt::Stmt, 64);
        assert_size_eq!(expr::Expr, 56);
    }
}
