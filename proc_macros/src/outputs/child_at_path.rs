use proc_macro2::TokenStream;

use quote::quote;
use syn::{Ident, Type};
use crate::PathlibImports;
use crate::utils::types::path_list::PathList;
use crate::utils::types::type_with_bounds::TypeWithBounds;

pub fn child_at_path_toks(bounded_child_type: TypeWithBounds, path_type: Type, unowned_struct_name: Ident, unowned_struct_derives: PathList, owned_struct_name: Ident, owned_struct_derives: PathList) -> TokenStream {
    // Generics
    let child_generics = bounded_child_type.generics();
    let child_generic_bounds = bounded_child_type.generic_bounds();
    // Types
    let child_type = bounded_child_type.bounded_ty;
    let lt = quote! {'a};
    // Derives
    let unowned_derive_paths = unowned_struct_derives.paths;
    let unowned_derives = quote! { #[derive(#(#unowned_derive_paths),*)] };
    let owned_derive_paths = owned_struct_derives.paths;
    let owned_derives = quote! { #[derive(#(#owned_derive_paths),*)] };
    // Imports from path_lib
    let imports = PathlibImports::default();
    let owned_obj_at_path = imports.owned_obj_at_path;

    // Build final tokens:
    quote! {
        // Unowned
        #unowned_derives
        pub struct #unowned_struct_name <#lt, #child_generic_bounds> {obj: &#lt #child_type, path: #path_type}
        // Owned
        #owned_derives
        pub struct #owned_struct_name<#child_generic_bounds> {obj: #child_type, path: #path_type}
        
        // Implement From<ObjAtPath> for ObjAtPath
        impl <#lt,#child_generic_bounds> From<#unowned_struct_name<#lt,#child_generics>> for #owned_struct_name<#child_generics> where #child_type: Clone {
            fn from(value: #unowned_struct_name<#lt,#child_generic_bounds>) -> Self
                { #owned_struct_name{obj: value.obj.clone(), path: value.path} }
        }
    }
}
