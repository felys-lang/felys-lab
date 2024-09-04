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
        let __pos__ = self.stream.mark();
        let __cache_type__ = CacheType::#cache #args;
        if let Some(cache) = self.cache.get(__pos__, __cache_type__) {
            let (end, cr) = cache;
            self.stream.reset(end);
            return cr.into()
        }
    };

    let cache = quote! {
        let result = || #rt #block();
        let __cache_result__ = CacheResult::#cache(result.clone());
        let __end__ = self.stream.mark();
        self.cache.insert(__pos__, __cache_type__, __end__, __cache_result__);
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
            impl From<CacheResult> for #inner {
                fn from(value: CacheResult) -> Self {
                    match value {
                        CacheResult::#cr(inner) => inner,
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
