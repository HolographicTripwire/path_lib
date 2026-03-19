use proc_macro_crate::{FoundCrate, crate_name};
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::Ident;

pub fn import(crate_id: &str, import_id: &str) -> TokenStream {
    let found_crate = crate_name(crate_id).expect(&format!("{} is present in `Cargo.toml`", crate_id));
    let import = syn::parse_str::<syn::Path>(&import_id).expect(&format!("Failed to convert '{}' to a path", import_id));
    //let import = Ident::new(&import_id, Span::call_site());

    match found_crate {
        FoundCrate::Itself => quote!( crate::#import ),
        FoundCrate::Name(name) => {
            let ident = Ident::new(&name, Span::call_site());
            quote!( #ident::#import )
        }
    }
}