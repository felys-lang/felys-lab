pub mod expr;
pub mod lit;
pub mod stmt;
pub mod pat;
pub mod item;

pub type ID = usize;


#[cfg(test)]
#[cfg(target_pointer_width = "64")]
mod test {
    use crate::*;

    #[test]
    fn size() {
        assert_eq!(size_of::<expr::Expr>(), 56);
        assert_eq!(size_of::<pat::Pat>(), 32);
        assert_eq!(size_of::<lit::Lit>(), 24);
        assert_eq!(size_of::<item::Item>(), 64);
        assert_eq!(size_of::<stmt::Stmt>(), 64);
    }
}
