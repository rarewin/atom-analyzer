extern crate proc_macro;

use proc_macro::TokenStream;

use quote::quote;

#[proc_macro_derive(Atom)]
pub fn atom_macro_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let gen = quote! {
        impl Atom for #name {
        }
    };
    gen.into()
}
