pub mod expr;
pub mod lit;
pub mod stmt;
pub mod pat;
pub mod item;


#[cfg(test)]
#[cfg(target_pointer_width = "64")]
mod test {
    use crate::*;

    #[test]
    fn size() {
        assert_eq!(size_of::<expr::Expr>(), 64);
        assert_eq!(size_of::<pat::Pat>(), 32);
        assert_eq!(size_of::<lit::Lit>(), 16);
        assert_eq!(size_of::<item::Item>(), 72);
        assert_eq!(size_of::<stmt::Stmt>(), 72);
    }
}
