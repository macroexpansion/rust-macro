extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(HelloWorld)]
pub fn hello_world_fn(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_hellow_world(&ast)
}

fn impl_hellow_world(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl HelloWorld for #name {
            fn hello() {
                println!("hello macro");
            }
        }
    };
    gen.into()
}
