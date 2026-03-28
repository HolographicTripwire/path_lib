use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{Type, TypePath};


pub mod type_with_bounds;
pub mod path_list;
pub mod ident_from_litstr;

pub fn check_selftype(t: &Type) -> bool {
    if let Type::Path(TypePath { qself: None, path }) = t
        { path.segments.len() == 1 && path.segments[0].ident == "Self" } else { false }    
}

pub fn quote_vec<T:Clone+ToTokens,Sep:ToTokens>(vec: &Vec<T>, separator: &Sep) -> TokenStream {
    let sum = vec.iter().cloned().map(|i| quote! { #i })
        .reduce(|acc, cur| quote! { #acc #separator #cur })
        .unwrap_or_else(|| quote! { });
    quote! { #sum }
}
