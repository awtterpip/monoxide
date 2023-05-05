use proc_macro::TokenStream;
use syn::token::Comma;
use syn::{ FnArg, ItemFn, Pat, Signature, parse_quote, Block};
use syn::punctuated::{Pair, Punctuated};
use quote::{quote, ToTokens};

#[proc_macro_attribute]
pub fn openxr(attr: TokenStream, item: TokenStream) -> TokenStream {
    let func: ItemFn = syn::parse(item).expect("Failed to parse function");
    let name = func.sig.ident.clone();
    let args: Punctuated<Pat, Comma> = func.sig.inputs.pairs().map(|arg| match arg {
        Pair::Punctuated(FnArg::Typed(arg), p) => Pair::Punctuated(*arg.pat.clone(), *p),
        Pair::End(FnArg::Typed(arg)) => Pair::End(*arg.pat.clone()),
        _ => panic!("Expected function but recieved method")
    }).collect();
    let new_func = ItemFn {
        sig: Signature {
            abi: Some(parse_quote!(extern "system")),
            ident: syn::parse(attr).expect("You must specify a function name"),
            output: parse_quote!(-> openxr_sys::Result),
            ..func.sig.clone()
        },
        block: Box::new(Block { 
            stmts: vec![
                parse_quote! {
                    match #name(#args) {
                        Ok(_) => openxr_sys::Result::SUCCESS,
                        Err(e) => e,
                    }
                }
            ],
            ..*func.block
        }),
        ..func.clone()
    };
    quote! {
        #[no_mangle]
        #new_func

        #func
    }.into()
}