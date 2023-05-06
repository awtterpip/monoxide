use proc_macro::TokenStream;
use syn::parse::Parse;
use syn::token::Bracket;
use syn::{ FnArg, ItemFn, Pat, Signature, parse_quote, Block, ItemStruct, Type, Ident, Token, bracketed, parse_macro_input};
use syn::punctuated::{Pair, Punctuated};
use quote::{quote};

#[proc_macro_attribute]
pub fn openxr(attr: TokenStream, item: TokenStream) -> TokenStream {
    let func: ItemFn = syn::parse(item).expect("Failed to parse function");
    let name = func.sig.ident.clone();
    let args: Punctuated<Pat, Token![,]> = func.sig.inputs.pairs().map(|arg| match arg {
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

#[proc_macro_attribute]
pub fn handle(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item: ItemStruct = syn::parse(item).unwrap();
    let attr: Type = syn::parse(attr).unwrap();
    let ty = item.ident.clone();
    quote! {
        #item

        #[doc(hidden)]
        static REGISTRY: once_cell::sync::Lazy<dashmap::DashMap<u64, #ty, std::hash::BuildHasherDefault<rustc_hash::FxHasher>>> = once_cell::sync::Lazy::new(|| std::default::Default::default());
        const COUNTER: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);
        
        #[doc(hidden)]
        impl crate::handle::Handle for #attr {
            type HandleType = #ty;

            fn raw(&self) -> u64 {
                self.into_raw()
            }

            fn get_mut<'a>(self) -> Result<dashmap::mapref::one::RefMut<'a, u64, Self::HandleType, std::hash::BuildHasherDefault<rustc_hash::FxHasher>>, openxr_sys::Result> {
                if self.raw() == 0 {
                    Err(openxr_sys::Result::ERROR_HANDLE_INVALID)
                } else {
                    REGISTRY.get_mut(&self.raw()).ok_or(openxr_sys::Result::ERROR_HANDLE_INVALID)
                }
            }

            fn get<'a>(self) -> Result<dashmap::mapref::one::Ref<'a, u64, Self::HandleType, std::hash::BuildHasherDefault<rustc_hash::FxHasher>>, openxr_sys::Result> {
                if self.raw() == 0 {
                    Err(openxr_sys::Result::ERROR_HANDLE_INVALID)
                } else {
                    REGISTRY.get(&self.raw()).ok_or(openxr_sys::Result::ERROR_HANDLE_INVALID)
                }
            }

            fn destroy(self) -> Result<(), openxr_sys::Result> {
                if self.raw() == 0 {
                    Err(openxr_sys::Result::ERROR_HANDLE_INVALID)
                } else {
                    match REGISTRY.remove(&self.raw()) {
                        Some(_) => Ok(()),
                        None => Err(openxr_sys::Result::ERROR_HANDLE_INVALID)
                    }
                }
            }

            fn is_null(self) -> bool {
                if self.raw() == 0 {
                    true
                } else {
                    REGISTRY.get(&self.raw()).is_none()
                }
            }
        }
    }.into()
}

struct OxrFns {
    name: Ident,
    no_inst: Array<Ident>,
    inst: Array<Ident>,
}

impl Parse for OxrFns {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            name: input.parse()?,
            no_inst: input.parse()?,
            inst: input.parse()?
        })
    }
}

struct Array<T> {
    _bracket: Bracket,
    items: Punctuated<T, Token![,]>,
}

impl<T: Parse> Parse for Array<T> {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let content;
        Ok(Self {
            _bracket: bracketed!(content in input),
            items: content.parse_terminated(T::parse, Token![,])?
        })
    }
}

#[proc_macro]
pub fn oxr_fns(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as OxrFns);
    let name = input.name;
    let inst = input.inst.items.iter();
    let no_inst = input.no_inst.items.iter();
    quote! {
        fn #name(instance: openxr_sys::Instance, name: &str) -> std::result::Result<openxr_sys::pfn::VoidFunction, openxr_sys::Result> {
            match (name, instance.is_null()) {
                #(
                    (stringify!(#no_inst), _) => Ok(unsafe { std::mem::transmute(#no_inst as usize)}),
                )*
                (_, true) => Err(openxr_sys::Result::ERROR_HANDLE_INVALID),
                #(
                    (stringify!(#inst), false) => Ok(unsafe { std::mem::transmute(#inst as usize)}),
                )*
                (_, false) => Err(openxr_sys::Result::ERROR_FUNCTION_UNSUPPORTED),
            }
        }
    }.into()
}