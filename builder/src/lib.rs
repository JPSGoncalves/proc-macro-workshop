use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::parse_macro_input;

#[proc_macro_derive(Builder)]
pub fn derive_builder(input: TokenStream) -> TokenStream {
    eprintln!("TOKENS: {:#}", input);
    let input = parse_macro_input!(input as syn::DeriveInput);

    let builder = create_builder(&input);

    let expand = quote! {
        #builder
    };

    eprintln!("Expanded TOKENS : {:#}", expand);

    TokenStream::new()
}

fn create_builder(input: &syn::DeriveInput) -> proc_macro2::TokenStream {
    let builder = format_ident!("{}Builder", input.ident);
    let vis = input.vis.to_token_stream();

    let expand = quote! {
        #vis struct #builder {}
    };

    expand
}

fn create_builder_fields(input: &syn::DeriveInput) -> proc_macro2::TokenStream {
    let mut builder_fields: Vec<syn::Field> = Vec::new();

    if let syn::Data::Struct(fields) = &input.data {
        for field in fields.fields.iter() {
                builder_fields.push(field.clone());
        }
    }

    if builder_fields.is_empty() {
        panic!("Derive Macro not applied to a struct");
    }

    unimplemented!()
}

fn extend_ogirinal(input: &syn::DeriveInput) -> TokenStream {
    unimplemented!()
}
