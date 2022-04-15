use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

///
/// Automatically implements `salmon_parser::Locatable`.
///
#[proc_macro_derive(Locatable)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let generics = input.generics;

    let expanded = quote! {
        impl #generics crate::ast::utils::Locatable for #name #generics {
            fn get_position(&self) -> &crate::error::FilePosition {
                &self.position
            }
        }
    };

    TokenStream::from(expanded)
}
