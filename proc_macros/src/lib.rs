use std::fmt::Display;

use proc_macro2::{Span, TokenStream, TokenTree};
use quote::{ToTokens, quote};
use syn::parse::{Parse, ParseStream};
use syn::token::Paren;
use syn::{parse_macro_input, punctuated::Punctuated, Token};
use syn::{
    Error, GenericParam, Ident, Lifetime, LitStr, Path, PredicateLifetime, PredicateType, Result as SynResult, TraitBound, Type, TypeParam, TypeParamBound, WhereClause, WherePredicate, bracketed, parenthesized
};
use proc_macro_crate::{crate_name, FoundCrate};

fn import(crate_id: &str, import_id: &str) -> TokenStream {
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

#[derive(Debug)]
struct  TypeWithBounds {
    bounded_ty: Type,
    where_clause: Option<WhereClause>
}

impl TypeWithBounds {
    fn pred_lifetimes(&self) -> Vec<PredicateLifetime> {
        match &self.where_clause {
            Some(where_clause) => where_clause.predicates
                .clone()
                .into_iter()
                .filter_map(|predicate| match predicate {
                    WherePredicate::Lifetime(predicate_lifetime) => Some(predicate_lifetime),
                    _ => None,
                })
                .collect(),
            None => vec![]
        }
    }
    fn pred_types(&self) -> Vec<PredicateType> {
        match &self.where_clause {
            Some(where_clause) => where_clause.predicates
                .clone()
                .into_iter()
                .filter_map(|predicate| match predicate {
                    WherePredicate::Type(predicate_type) => Some(predicate_type),
                    _ => None,
                })
                .collect(),
            None => vec![]
        }
    }

    fn generics(&self) -> TokenStream {
        let lifetimes: Vec<Lifetime> = self.pred_lifetimes()
            .into_iter()
            .map(|lifetime| lifetime.lifetime)
            .collect();
        let lifetimes_punctuated = quote!{#(#lifetimes),*};

        let types: Vec<Type> = self
            .pred_types()
            .into_iter()
            .map(|pred_type| pred_type.bounded_ty )
            .collect();
        let types_punctuated = quote!{#(#types),*};
        if lifetimes.len() > 0 && types.len() > 0 {
            quote!{#lifetimes_punctuated,#types_punctuated}
        } else {
            quote!{#lifetimes_punctuated #types_punctuated}
        }
    }

    fn generic_bounds(&self) -> TokenStream {
        match &self.where_clause {
            Some(where_clause) => {
                let predicates = &where_clause.predicates;
                quote! { #predicates }
            }, None => quote! {},
        }
    }
}

impl Parse for TypeWithBounds {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let obj_type = input.parse()?;
        let where_clause = {
            if input.lookahead1().peek(Token![where]) { Some(input.parse()?) }
            else { None }
        };
        return Ok(Self { bounded_ty: obj_type, where_clause });
    }
}
impl Display for TypeWithBounds {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let obj_type = self.bounded_ty.clone();
        let where_clause = self.where_clause.clone();
        write!(f, "{}", quote!{#obj_type #where_clause})
    }
}

struct PathListInner {
    paths: Vec<Path>
}

impl Parse for PathListInner {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let mut paths = Vec::new();
        loop {
            paths.push(input.parse()?);
            if input.is_empty() { break; }
            let _: Token![,] = input.parse()?;
        }
        Ok(PathListInner{ paths })
    }
}

struct PathList {
    paths: Vec<Path>
}
impl Parse for PathList {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let Some(TokenTree::Group(tree)) = input.parse()? else { return SynResult::Err(syn::Error::new(Span::call_site(), "Parameter must be enclosed in brackets") ) };
        if tree.delimiter() != proc_macro2::Delimiter::Bracket { return SynResult::Err(syn::Error::new(Span::call_site(), "Parameter must be enclosed in brackets")) };
        let inner_toks = tree.stream().into();
        let inner: PathListInner = syn::parse2(inner_toks)?;
        Ok(PathList { paths: inner.paths })
    }
}

struct MacroInput {
    obj_type_paren: Paren,
    obj_type: TypeWithBounds,
    path_type: Type,
    unowned_name: Ident,
    unowned_derives: PathList,
    owned_name: Ident,
    owned_derives: PathList,
}

fn transform_syn_err<T, F: FnOnce(String) -> String>(r: SynResult<T>, f: F) -> SynResult<T> {
    r.map_err(|e| Error::new(e.span(), f(e.to_string())))
}

impl Parse for MacroInput {
    fn parse(input: ParseStream) -> SynResult<Self> {
        
        // Parse first item: either a type (syn::Type) or a generic param (e.g. T: Clone)
        let content;
        let obj_type_paren = parenthesized!(content in input);//transform_syn_err(input.parse(), |msg| format!{"Error while parsing obj_type (arg 1): {}", msg})?;
        let obj_type = content.parse()?;
        // Expect comma
        let _: Token![,] = transform_syn_err(input.parse(), |_msg| format!{"Error while parsing obj_type (arg 1): Missing comma after argument"})?;
        // parse second type
        let path_type: Type = transform_syn_err(input.parse(), |msg| format!{"Error while parsing path_type (arg 2): {}", msg})?;
        // Expect comma
        let _: Token![,] = transform_syn_err(input.parse(), |_msg| format!{"Error while parsing path_type (arg 2): Missing comma after argument"})?;

        // parse name as string literal, then turn into Ident (validate it's a valid identifier)
        let unowned_name_literal: LitStr = transform_syn_err(input.parse(), |msg| format!{"Error while parsing unowned_name (arg 3): {}", msg})?;
        let unowned_name = Ident::new(&unowned_name_literal.value(), Span::call_site());
        // Expect comma
        let _: Token![,] = transform_syn_err(input.parse(), |_msg| format!{"Error while parsing unowned_name (arg 3): Missing comma after argument"})?;
        let unowned_derives = transform_syn_err(input.parse(), |msg| format!{"Error while parsing unowned_derives (arg 4): {}", msg})?;
        let _: Token![,] = transform_syn_err(input.parse(), |_msg| format!{"Error while parsing unowned_derives (arg 4): Missing comma after argument"})?;
        
        // parse name as string literal, then turn into Ident (validate it's a valid identifier)
        let owned_name_literal: LitStr = transform_syn_err(input.parse(), |msg| format!{"Error while parsing owned_name (arg 5): {}", msg})?;
        let owned_name = Ident::new(&owned_name_literal.value(), Span::call_site());
        let _: Token![,] = transform_syn_err(input.parse(), |_msg| format!{"Error while parsing owned_name (arg 5): Missing comma after argument"})?;
        let owned_derives = transform_syn_err(input.parse(), |msg| format!{"Error while parsing owned_derives (arg 6): {}", msg})?;

        // Convert string into an Ident. Validate it is a valid Rust identifier by using Ident::new.
        // Note: Ident::new will accept many things; we assume user gives a valid identifier string.
        Ok(MacroInput { obj_type_paren, obj_type, path_type, unowned_name, unowned_derives, owned_name, owned_derives })
    }
}

#[proc_macro]
pub fn generate_obj_at_path_wrappers(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as MacroInput);

    // Names of structs
    let owned_struct = input.owned_name;
    let unowned_struct = input.unowned_name;
    // Generics
    let generics = input.obj_type.generics();
    let generic_bounds = input.obj_type.generic_bounds();
    // Types
    let obj_type = input.obj_type.bounded_ty;
    let path_type = input.path_type;
    let lt = quote! {'a};
    // Derives
    let unowned_paths = input.unowned_derives.paths;
    let unowned_derives = quote! { #[derive(#(#unowned_paths),*)] };
    let owned_paths = input.owned_derives.paths;
    let owned_derives = quote! { #[derive(#(#owned_paths),*)] };
    // Imports from path_lib
    let crate_id = "path_lib";
    let path = import(crate_id, "paths::Path");
    let obj_at_path = import(crate_id, "obj_at_path::ObjAtPath");
    let owned_obj_at_path = import(crate_id, "obj_at_path::OwnedObjAtPath");
    let has_descendants = import(crate_id, "HasDescendants");

    // Build final tokens:
    let expanded = quote! {
        // Unowned
        #unowned_derives
        pub struct #unowned_struct <#lt, #generic_bounds> (#obj_at_path<'a, #obj_type, #path_type>);
        // Implement base
        impl <#lt, #generic_bounds> #unowned_struct<#lt, #generics> {
            pub fn from_inner(obj_at: &#lt #obj_type, path: #path_type) -> Self
                { Self(#obj_at_path::from_inner(obj_at, path)) }
            pub fn from_outer<Joiner,O: #has_descendants<#lt,#path_type,Joiner,#obj_type>>(obj_in: &#lt O, path: #path_type) -> Result<Self,()>
                { #obj_at_path::from_outer(obj_in, path).map(|o| Self(o)) }
            
            pub fn obj(&#lt self) -> &#lt #obj_type { self.0.obj() }
            pub fn path(&#lt self) -> &#lt #path_type { self.0.path() } 

            pub fn into_obj_and_path(self) -> (&#lt #obj_type, #path_type) { self.0.into_obj_and_path() }
            pub fn into_owned(self) -> #owned_struct<#generics> { #owned_struct(self.0.into_owned()) }

            pub fn replace_path<NewPath: #path>(self, function: impl Fn(#path_type) -> NewPath) -> #obj_at_path<'a,#obj_type,NewPath>
                { self.0.replace_path(function) }
        }
        // Implement From<ObjAtPath>
        impl <#lt, #generic_bounds> From<#obj_at_path<#lt,#obj_type,#path_type>> for #unowned_struct<#lt,#generics> {
            fn from(value: #obj_at_path<#lt,#obj_type,#path_type>) -> Self
                { #unowned_struct(value) }
        }

        // Owned
        #owned_derives
        pub struct #owned_struct<#generic_bounds> (#owned_obj_at_path<#obj_type, #path_type>);
        impl <#generic_bounds> #owned_struct<#generics> {
            pub fn from_inner(obj_at: #obj_type, path: #path_type) -> Self
                { Self(#owned_obj_at_path::from_inner(obj_at, path))}
            pub fn from_outer<#lt, Joiner,O: #has_descendants<#lt, #path_type,Joiner,#obj_type>>(obj_in: &#lt O, path: #path_type) -> Result<Self,()> where #obj_type: #lt
                { #owned_obj_at_path::from_outer(obj_in, path).map(|o| Self(o)) }
            
            pub fn obj<#lt>(&#lt self) -> &#lt #obj_type where #obj_type: #lt { &self.0.obj() }
            pub fn path<#lt>(&#lt self) -> &#lt #path_type { &self.0.path() } 
            pub fn into_obj_and_path(self) -> (#obj_type,#path_type) { self.0.into_obj_and_path() }
        }
        // Implement From<OwnedObjAtPath>
        impl <#generic_bounds> From<#owned_obj_at_path<#obj_type,#path_type>> for #owned_struct<#generics> {
            fn from(value: #owned_obj_at_path<#obj_type,#path_type>) -> Self
                { #owned_struct(value) }
        }
        
    };

    //panic!("{}", TokenStream::from(expanded).to_string());
    return proc_macro::TokenStream::from(expanded);
}
