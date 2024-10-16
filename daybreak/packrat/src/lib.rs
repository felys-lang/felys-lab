mod helper;

use crate::helper::*;
use proc_macro::TokenStream;

/// Cache the result of this sub-expression
///
/// Put `#[daybreak::memoize(AST)]` on top of the method,
/// where `AST` is the variant name in `Self::CT` and `Self::CR`.
#[proc_macro_attribute]
pub fn memoize(meta: TokenStream, body: TokenStream) -> TokenStream {
    memoize_helper(meta, body)
}

/// Allow left-recursion in this sub-expression, caching required
///
/// Put `#[daybreak::lecursion(AST)]` on top of the method,
/// where `AST` is the variant name in `Self::CT` and `Self::CR`.
#[proc_macro_attribute]
pub fn lecursion(meta: TokenStream, body: TokenStream) -> TokenStream {
    lecursion_helper(meta, body)
}
