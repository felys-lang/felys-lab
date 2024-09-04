use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Expr, FnArg, ItemFn, Meta, MetaNameValue, Pat, PatType};

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

    let args = signature.inputs.iter()
        .filter_map(|x| match x {
            FnArg::Typed(PatType { pat, .. }) => Some(pat),
            _ => None
        })
        .collect::<Vec<&Box<Pat>>>();

    let args = if args.is_empty() {
        quote! {}
    } else {
        quote! { (#(#args),*) }
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
