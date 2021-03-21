extern crate proc_macro;

use proc_macro::TokenStream;
use syn::parse::Parser;
use syn::{parse_macro_input, AttributeArgs, DeriveInput, Fields, ItemStruct};

use quote::quote;

#[proc_macro_derive(Atom)]
pub fn atom_macro_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;
    let gen = quote! {
        impl Atom for #name {
        }
    };
    gen.into()
}

#[proc_macro_attribute]
pub fn atom(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut item_struct = parse_macro_input!(input as ItemStruct);
    let attr_args = parse_macro_input!(args as AttributeArgs);

    let name = &item_struct.ident;

    let mut opt_version = false;

    for arg in attr_args {
        if let syn::NestedMeta::Meta(syn::Meta::Path(path)) = arg {
            for seg in path.segments {
                if seg.ident == "version" {
                    opt_version = true;
                }
            }
        }
    }

    if let Fields::Named(ref mut fields) = item_struct.fields {
        fields.named.insert(
            0,
            syn::Field::parse_named
                .parse2(quote! { pub atom_head: AtomHead })
                .unwrap(),
        );

        if opt_version {
            fields.named.insert(
                1,
                syn::Field::parse_named
                    .parse2(quote! { pub atom_version: u8 })
                    .unwrap(),
            );
            fields.named.insert(
                2,
                syn::Field::parse_named
                    .parse2(quote! { pub atom_flags: [u8; 3] })
                    .unwrap(),
            );
        }
    }

    let gen = quote! {
        #item_struct

        impl Atom for #name {}
    };

    gen.into()
}
