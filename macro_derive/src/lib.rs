use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(MacroTrait)]
pub fn test_macro_trait_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let name = &ast.ident;

    let expanded = quote! {
        impl MacroTrait for #name {
            fn test(){
                println!("Test macro from {}", stringify!(#name));
            }
        }
    };

    TokenStream::from(expanded)
}
