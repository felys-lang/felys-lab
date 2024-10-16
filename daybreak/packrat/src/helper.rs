use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn, Path};

pub fn memoize_helper(meta: TokenStream, body: TokenStream) -> TokenStream {
    let meta = parse_macro_input!(meta as Path);
    let body = parse_macro_input!(body as ItemFn);

    let sig = &body.sig;
    let rt = &sig.output;
    let block = &body.block;
    let vis = &body.vis;

    let mut pa = sig.inputs.iter().skip(1).peekable();
    let args = if pa.peek().is_some() {
        quote! { (#(#pa),*) }
    } else {
        quote! {}
    };

    quote!(
        #vis #sig {
            let pos = self.stream.cursor;
            let mode = self.stream.strict;
            let ct = Self::CT::#meta #args;
            if let Some(memo) = self.memo.get(pos, mode, ct) {
                let (end, cr) = memo;
                self.stream.cursor = end;
                return cr.into()
            }
            let result = || #rt #block();
            let ct = Self::CT::#meta #args;
            let end = self.stream.cursor;
            let cr = Self::CR::#meta(result.clone());
            self.memo.insert(pos, mode, ct, end, cr);
            result
        }
    ).into()
}

pub fn lecursion_helper(meta: TokenStream, body: TokenStream) -> TokenStream {
    let meta = parse_macro_input!(meta as Path);
    let body = parse_macro_input!(body as ItemFn);

    let sig = &body.sig;
    let rt = &sig.output;
    let block = &body.block;
    let vis = &body.vis;

    let mut pa = sig.inputs.iter().skip(1).peekable();
    let args = if pa.peek().is_some() {
        quote! { (#(#pa),*) }
    } else {
        quote! {}
    };

    quote!(
        #[::daybreak::memoize(#meta)]
        #vis #sig {
            let pos = self.stream.cursor;
            let mut cr = Self::CR::#meta(None);
            let mut end = pos;
            loop {
                let mode = self.stream.strict;
                let ct = Self::CT::#meta #args;
                self.memo.insert(pos, mode, ct, end, cr.clone());
                let res = || #rt #block();
                if end < self.stream.cursor {
                    cr = Self::CR::#meta(res);
                    end = self.stream.cursor;
                    self.stream.cursor = pos;
                } else {
                    self.stream.cursor = end;
                    break cr.into();
                }
            }
        }
    ).into()
}
