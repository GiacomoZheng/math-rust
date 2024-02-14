use quote::quote;
// use syn::spanned::Spanned;
use syn::{
    parse_macro_input, parse_quote,
    DeriveInput, 
    GenericParam, Generics, 
};

#[proc_macro_derive(BasicObject)]
pub fn derive_basic_object(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Parse the input tokens into a syntax tree.
    let input = parse_macro_input!(input as DeriveInput);

    // Used in the quasi-quotation below as `#name`.
    let name = input.ident;

    // Add a bound `T: HeapSize` to every type parameter T.
    let generics = add_trait_bounds_basic(input.generics);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let expanded = quote! {
        // The generated impl.
        impl #impl_generics alias::BasicObject for #name #ty_generics #where_clause {}
    };

    // Hand the output tokens back to the compiler.
    proc_macro::TokenStream::from(expanded)
}
fn add_trait_bounds_basic(mut generics: Generics) -> Generics {
    for param in &mut generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param.bounds.push(parse_quote!(alias::BasicObject));
        }
    }
    generics
}

#[proc_macro_derive(BasicObjectCopy)]
pub fn derive_basic_object_copy(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Parse the input tokens into a syntax tree.
    let input = parse_macro_input!(input as DeriveInput);

    // Used in the quasi-quotation below as `#name`.
    let name = input.ident;

    // Add a bound `T: HeapSize` to every type parameter T.
    let generics = add_trait_bounds_copy(input.generics);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let expanded = quote! {
        // The generated impl.
        impl #impl_generics alias::BasicObjectCopy for #name #ty_generics #where_clause {}
    };

    // Hand the output tokens back to the compiler.
    proc_macro::TokenStream::from(expanded)
}
fn add_trait_bounds_copy(mut generics: Generics) -> Generics {
    for param in &mut generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param.bounds.push(parse_quote!(alias::BasicObjectCopy));
        }
    }
    generics
}
