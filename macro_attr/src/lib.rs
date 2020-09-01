use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_attribute]
pub fn test_macro_attr(metadata: TokenStream, input: TokenStream) -> TokenStream {

    let func = Ident::new(&metadata.to_string(), Span::call_site());
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;

    let expanded = quote! {
        #ast

        impl MacroTrait for #name {
            fn test(){
                #func()
            }
        }
    };

    TokenStream::from(expanded)
}
