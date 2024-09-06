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

    let store = quote! {
        let __m_result = || #rt #block();
        let __m_cache_result = crate::ast::CacheResult::#cache(__m_result.clone());
        let __m_end = self.stream.mark();
        self.cache.insert(__m_pos, __m_cache_type, __m_end, __m_cache_result);
        __m_result
    };

    let extended = quote! {
        #vis #signature {
            #fast
            #store
        }
    };

    extended.into()
}


#[proc_macro_attribute]
pub fn lecursion(meta: TokenStream, body: TokenStream) -> TokenStream {
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

    let body = quote! {
        let __l_pos = self.stream.mark();
        let __l_cache_type = crate::ast::CacheType::#cache #args;
        let mut __l_cache_result = crate::ast::CacheResult::#cache(None);
        let mut __l_end = __l_pos;
        loop {
            self.cache.insert(__l_pos, __l_cache_type, __l_end, __l_cache_result.clone());
            let res = || #rt #block();
            if __l_end < self.stream.mark() {
                __l_cache_result = crate::ast::CacheResult::#cache(res);
                __l_end = self.stream.mark();
                self.stream.jump(__l_pos);
            } else {
                self.stream.jump(__l_end);
                break __l_cache_result.into();
            }
        }
    };

    let extended = quote! {
        #[pegmacro::memoize(cache = #cache)]
        #vis #signature {
            #body
        }
    };

    extended.into()
}


#[proc_macro_derive(CR)]
pub fn impl_cr_into_inner(body: TokenStream) -> TokenStream {
    let body = parse_macro_input!(body as DeriveInput);

    let variants = match &body.data {
        Data::Enum(e) => &e.variants,
        _ => panic!("this only works with enum")
    };

    let from = variants.iter().map(|x| {
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

    let display = variants.iter().map(|x| {
        let cr = &x.ident;
        quote! {
            crate::ast::CacheResult::#cr(x) => match x {
                Some(i) => write!(f, "{}", i),
                None => write!(f, "None"),
            }
        }
    });

    let extended = quote! {
        #(#from)*
        impl std::fmt::Display for crate::ast::CacheResult {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    #(#display)*
                }
            }
        }
    };

    extended.into()
}
