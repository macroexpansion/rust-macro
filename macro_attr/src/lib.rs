extern crate proc_macro;

use proc_macro::TokenStream;

use quote::quote;
use syn::{self, Item};

#[proc_macro_attribute]
pub fn macro_attribute(attr: TokenStream, item: TokenStream) -> TokenStream {
    println!("attr: {attr}");
    println!("item: {item}");

    // let attr_ast: Item = syn::parse(attr).unwrap();
    // if let Item::Verbatim(tokens) = attr_ast {
    //     println!("{}", 1);
    // }

    let item_ast: Item = syn::parse(item).unwrap();
    let func = if let Item::Fn(func) = item_ast {
        Some(func)
    } else {
        None
    };
    let func = func.unwrap();

    let fn_ident = func.sig.ident;
    let gen = quote! {
        fn #fn_ident() {
            println!("this is {} function", stringify!(#fn_ident));
        }
    };
    TokenStream::from(gen)
}
