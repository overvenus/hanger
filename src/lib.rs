extern crate proc_macro;

#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use syn::parse;
use syn::punctuated::Punctuated;

#[proc_macro_attribute]
pub fn hook(attr: TokenStream, input: TokenStream) -> TokenStream {
    let hooks = syn::parse_macro_input!(attr as Hooks);
    let mut function = syn::parse_macro_input!(input as syn::ItemFn);

    let before: Box<syn::Block> = if let Some(func) = hooks.before {
        Box::new(syn::parse_quote!({ #func() }))
    } else {
        Box::new(syn::parse_quote!({}))
    };
    let after: Box<syn::Block> = if let Some(func) = hooks.after {
        Box::new(syn::parse_quote!({ #func() }))
    } else {
        Box::new(syn::parse_quote!({}))
    };

    let body = function.block;
    function.block = Box::new(syn::parse_quote!({
        #before { #body } #after
    }));
    TokenStream::from(quote!(#function))
}

struct Hooks {
    before: Option<syn::TypePath>,
    after: Option<syn::TypePath>,
}

mod pk {
    use super::*;

    syn::custom_keyword!(before);
    syn::custom_keyword!(after);

    pub enum Arg {
        Before {
            b: before,
            eq: syn::Token![=],
            func: syn::TypePath,
        },
        After {
            a: after,
            eq: syn::Token![=],
            func: syn::TypePath,
        },
    }

    impl parse::Parse for Arg {
        fn parse(input: parse::ParseStream) -> parse::Result<Self> {
            let lookahead = input.lookahead1();
            if lookahead.peek(before) {
                Ok(Arg::Before {
                    b: input.parse::<before>()?,
                    eq: input.parse::<syn::Token![=]>()?,
                    func: input.parse()?,
                })
            } else {
                Ok(Arg::After {
                    a: input.parse::<after>()?,
                    eq: input.parse::<syn::Token![=]>()?,
                    func: input.parse()?,
                })
            }
        }
    }
}

impl parse::Parse for Hooks {
    fn parse(input: parse::ParseStream) -> parse::Result<Self> {
        let mut hb = None;
        let mut ha = None;
        let parser = Punctuated::<pk::Arg, syn::Token![,]>::parse_terminated;
        if let Ok(args) = parser(input) {
            for arg in args.iter() {
                match arg {
                    pk::Arg::After { func, .. } => ha = Some(func.clone()),
                    pk::Arg::Before { func, .. } => hb = Some(func.clone()),
                }
            }
        }

        if hb.is_none() && ha.is_none() {
            hb = Some(input.parse::<syn::TypePath>()?);
        }

        Ok(Hooks {
            after: ha,
            before: hb,
        })
    }
}
