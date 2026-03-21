use proc_macro2::Ident;
use syn::LitStr;

pub fn ident_from_literal(litstr: LitStr) -> Ident
    { Ident::new(&litstr.value(), litstr.span()) }
