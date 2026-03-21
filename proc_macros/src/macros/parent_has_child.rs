use proc_macro2::Span;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::token::Paren;
use syn::{Token, parse_macro_input};
use syn::{
    Ident, LitStr, {Result as SynResult}, Type, parenthesized
};

use crate::{PathlibImports, transform_syn_err};
use crate::utils::types::type_with_bounds::TypeWithBounds;

struct MacroInput {
    _child_type_paren: Paren,
    child_type: TypeWithBounds,
    path_type: Type,
    unowned_name: Type,
    owned_name: Type,
    get_child_paths: Ident,
    get_child: Ident,
}

impl Parse for MacroInput {
    fn parse(input: ParseStream) -> SynResult<Self> {
        // Parse first item: either a type (syn::Type) or a generic param (e.g. T: Clone)
        let content;
        let parent_type_paren = parenthesized!(content in input);//transform_syn_err(input.parse(), |msg| format!{"Error while parsing obj_type (arg 1): {}", msg})?;
        let parent_type = content.parse()?;
        // Expect comma
        let _: Token![,] = transform_syn_err(input.parse(), |_msg| format!{"Error while parsing parent_type (arg 1): Missing comma after argument"})?;
        
        // parse second type
        let unowned_type: Type = transform_syn_err(input.parse(), |msg| format!{"Error while parsing unowned_type (arg 2): {}", msg})?;
        // Expect comma
        let _: Token![,] = transform_syn_err(input.parse(), |_msg| format!{"Error while parsing unowned_type (arg 2): Missing comma after argument"})?;

        // parse third type
        let owned_type: Type = transform_syn_err(input.parse(), |msg| format!{"Error while parsing owned_type (arg 3): {}", msg})?;
        // Expect comma
        let _: Token![,] = transform_syn_err(input.parse(), |_msg| format!{"Error while parsing owned_type (arg 3): Missing comma after argument"})?;

        // parse third type
        let owned_type: Type = transform_syn_err(input.parse(), |msg| format!{"Error while parsing owned_type (arg 3): {}", msg})?;
        // Expect comma
        let _: Token![,] = transform_syn_err(input.parse(), |_msg| format!{"Error while parsing owned_type (arg 3): Missing comma after argument"})?;

        // parse name as string literal, then turn into Ident (validate it's a valid identifier)
        let owned_name_literal: LitStr = transform_syn_err(input.parse(), |msg| format!{"Error while parsing owned_name (arg 5): {}", msg})?;
        let owned_name = Ident::new(&owned_name_literal.value(), Span::call_site());
        let _: Token![,] = transform_syn_err(input.parse(), |_msg| format!{"Error while parsing owned_name (arg 5): Missing comma after argument"})?;
        let owned_derives = transform_syn_err(input.parse(), |msg| format!{"Error while parsing owned_derives (arg 6): {}", msg})?;

        // Convert string into an Ident. Validate it is a valid Rust identifier by using Ident::new.
        // Note: Ident::new will accept many things; we assume user gives a valid identifier string.
        Ok(MacroInput { _parent_type_paren: parent_type_paren, parent_type, path_type: unowned_type, unowned_name, unowned_derives, owned_name, owned_derives })
    }
}

pub fn generate_obj_at_path_wrappers(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as MacroInput);

    // Names of structs
    let owned_struct_name = input.owned_name;
    let unowned_struct_name = input.unowned_name;
    // Generics
    let child_generics = input.parent_type.generics();
    let child_generic_bounds = input.parent_type.generic_bounds();
    // Types
    let child_type = input.parent_type.bounded_ty;
    let path_type = input.path_type;
    let lt = quote! {'a};
    // Derives
    let unowned_derive_paths = input.unowned_derives.paths;
    let unowned_derives = quote! { #[derive(#(#unowned_derive_paths),*)] };
    let owned_derive_paths = input.owned_derives.paths;
    let owned_derives = quote! { #[derive(#(#owned_derive_paths),*)] };
    // Imports from path_lib
    let imports = PathlibImports::default();
    let owned_obj_at_path = imports.owned_obj_at_path;

    // Build final tokens:
    let expanded = quote! {
        // Unowned
        #unowned_derives
        pub struct #unowned_struct_name <#lt, #child_generic_bounds> {obj: &#lt #child_type, path: #path_type};
        // Owned
        #owned_derives
        pub struct #owned_struct_name<#child_generic_bounds> {obj: #child_type, path: #path_type};
        
        // Implement From<ObjAtPath> for ObjAtPath
        impl <#lt,#child_generic_bounds> From<#unowned_struct_name<#lt,#child_generics>> for #owned_struct_name<#child_generics> where #child_type: Clone {
            fn from(value: #owned_obj_at_path<#child_type,#path_type>) -> Self
                { #owned_struct_name{obj: value.obj.clone(), path: value.path} }
        }
    };

    //panic!("{}", TokenStream::from(expanded).to_string());
    return proc_macro::TokenStream::from(expanded);
}
