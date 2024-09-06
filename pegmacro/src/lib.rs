use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Expr, Fields, FieldsUnnamed, FnArg, ItemFn, Meta, MetaNameValue, PatType};

#[proc_macro_attribute]
pub fn memoize(meta: TokenStream, body: TokenStream) -> TokenStream {
    let meta = parse_macro_input!(meta as Meta);
    let body = parse_macro_input!(body as ItemFn);

    let signature = body.sig;
    let rt = &signature.output;
    let block = body.block;
    let vis = body.vis;

    let cache = match meta.require_name_value() {
        Ok(MetaNameValue {
               path,
               value: Expr::Path(ident), ..
           }) if path.is_ident("cache") => ident,
        _ => panic!("cache argument is missing"),
    };

    let mut args = signature.inputs.iter()
        .filter_map(|x| match x {
            FnArg::Typed(PatType { pat, .. }) => Some(pat),
            _ => None
        })
        .peekable();

    let args = if args.peek().is_some() {
        quote! { (#(#args),*) }
    } else {
        quote! {}
    };

    let fast = quote! {
        let __m_pos = self.stream.mark();
        let __m_cache_type = crate::ast::CacheType::#cache #args;
        if let Some(cache) = self.cache.get(__m_pos, __m_cache_type) {
            let (__m_end, __m_cache_result) = cache;
            self.stream.jump(__m_end);
            return __m_cache_result.into()
        }
    };

    let cache = quote! {
        let result = || #rt #block();
        let __m_cache_result = crate::ast::CacheResult::#cache(result.clone());
        let __m_end = self.stream.mark();
        self.cache.insert(__m_pos, __m_cache_type, __m_end, __m_cache_result);
        result
    };

    let extended = quote! {
        #vis #signature {
            #fast
            #cache
        }
    };

    extended.into()
}


#[proc_macro_derive(Unwrap)]
pub fn impl_cr_into_inner(body: TokenStream) -> TokenStream {
    let body = parse_macro_input!(body as DeriveInput);

    let variants = match &body.data {
        Data::Enum(e) => &e.variants,
        _ => panic!("this only works with enum")
    };

    let variants = variants.iter().map(|x| {
        let inner = match &x.fields {
            Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => unnamed,
            _ => panic!("enums need to be unnamed")
        };
        let cr = &x.ident;
        quote! {
            impl From<crate::ast::CacheResult> for #inner {
                fn from(value: crate::ast::CacheResult) -> Self {
                    match value {
                        crate::ast::CacheResult::#cr(inner) => inner,
                        _ => panic!("cache unmatched")
                    }
                }
            }
        }
    });

    let extended = quote! {
        #(#variants)*
    };

    extended.into()
}
