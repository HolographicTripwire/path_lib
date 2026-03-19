mod utils;

use proc_macro2::Span;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::token::Paren;
use syn::{parse_macro_input, Token};
use syn::{
    Error, Ident, LitStr, {Result as SynResult}, Type, parenthesized
};

use crate::utils::imports::import;
use crate::utils::types::path_list::PathList;
use crate::utils::types::type_with_bounds::TypeWithBounds;

struct MacroInput {
    _obj_type_paren: Paren,
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
        Ok(MacroInput { _obj_type_paren: obj_type_paren, obj_type, path_type, unowned_name, unowned_derives, owned_name, owned_derives })
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
    let path_primitive = import(crate_id, "paths::PathPrimitive");
    let path_pair = import(crate_id, "paths::PathPair");
    let obj_at_path = import(crate_id, "obj_at_path::ObjAtPath");
    let owned_obj_at_path = import(crate_id, "obj_at_path::OwnedObjAtPath");
    let has_children = import(crate_id, "HasChildren");
    let has_descendants = import(crate_id, "HasDescendants");

    // Build final tokens:
    let expanded = quote! {
        // Unowned
        #unowned_derives
        pub struct #unowned_struct <#lt, #generic_bounds> (#obj_at_path<#lt, #obj_type, #path_type>);
        // Implement base
        impl <#lt, #generic_bounds> #unowned_struct<#lt, #generics> {
            pub fn from_inner(obj_at: &#lt #obj_type, path: #path_type) -> Self
                { Self(#obj_at_path::from_inner(obj_at, path)) }
            pub fn from_outer<Joiner,O: #has_descendants<#lt,#path_type,Joiner,#obj_type>>(obj_in: &#lt O, path: #path_type) -> Result<Self,()>
                { #obj_at_path::from_outer(obj_in, path).map(|o| Self(o)) }
            
            pub fn obj(&#lt self) -> &#lt #obj_type { self.0.obj() }
            pub fn path(&#lt self) -> &#lt #path_type { self.0.path() } 

            pub fn into_obj_and_path(self) -> (&#lt #obj_type, #path_type) { self.0.into_obj_and_path() }
            pub fn into_located_children<Child,PathToAppend>(self) -> impl IntoIterator<Item = #obj_at_path<#lt,Child,#path_pair<#path_type,PathToAppend>>>
            where #obj_type: #has_children<PathToAppend,Child>, PathToAppend: #path_primitive, Child: #lt {
                self.0.into_located_children()
            }
            pub fn into_owned(self) -> #owned_struct<#generics> { #owned_struct(self.0.into_owned()) }

            pub fn replace_path<NewPath: #path>(self, function: impl Fn(#path_type) -> NewPath) -> #obj_at_path<#lt,#obj_type,NewPath>
                { self.0.replace_path(function) }
        }
        // Implement Appendable
        impl <#lt, #generic_bounds> #unowned_struct<#lt, #generics> {
            pub fn append<PathToAppend:#path,J,NewObj>(&self, subpath: PathToAppend) -> Result<#obj_at_path<#lt,NewObj,#path_pair<#path_type,PathToAppend>>,()> where #obj_type: #has_descendants<#lt,PathToAppend,J,NewObj>
                { self.0.append(subpath) }
            pub fn append_owned<PathToAppend:#path,J:Clone,NewObj:Clone>(&self, subpath: PathToAppend) -> Result<#owned_obj_at_path<NewObj,#path_pair<#path_type,PathToAppend>>,()> where #obj_type: #has_descendants<#lt,PathToAppend,J,NewObj>
                { self.0.append_owned(subpath) }
        }
        // Implement HasChildren
        impl <#lt, #generic_bounds> #unowned_struct<#lt, #generics> {
            pub fn valid_primitive_paths<Joiner:#path_primitive,Child>(&self) -> impl IntoIterator<Item = Joiner> where #obj_type: #has_children<Joiner,Child>  
                { self.0.valid_primitive_paths() }
            
            pub fn get_child<Joiner:#path_primitive,Child>(&self, path: &Joiner) -> Result<&Child,()> where #obj_type: #has_children<Joiner,Child>
                { self.0.get_child(path) }
            pub fn get_children<Joiner:#path_primitive,Child:#lt>(&self) -> impl IntoIterator<Item = &Child> where #obj_type: #has_children<Joiner,Child>
                { self.0.get_children() }
            
            pub fn get_child_owned<Joiner:#path_primitive,Child:Clone>(&self, path: &Joiner) -> Result<Child,()> where #obj_type: #has_children<Joiner,Child>
                { self.0.get_child_owned(path) }
            pub fn get_children_owned<Joiner:#path_primitive,Child:Clone>(&self) -> impl IntoIterator<Item = Child> where #obj_type: #has_children<Joiner,Child>
                { self.0.get_children_owned() }

            pub fn get_located_child<Joiner:#path_primitive,Child>(&#lt self, path: Joiner) -> Result<#obj_at_path<#lt,Child,#path_pair<#path_type,Joiner>>,()> where #obj_type: #has_children<Joiner,Child>
                { self.0.get_located_child(path) }
            pub fn get_located_children<Joiner:#path_primitive,Child:#lt>(&#lt self) -> impl IntoIterator<Item = #obj_at_path<#lt,Child,#path_pair<#path_type,Joiner>>> where #obj_type: #has_children<Joiner,Child>
                { self.0.get_located_children() }

            pub fn get_located_child_owned<Joiner:#path_primitive,Child:Clone>(&self, path: Joiner) -> Result<#owned_obj_at_path<Child,#path_pair<#path_type,Joiner>>,()> where #obj_type: #has_children<Joiner,Child>
                { self.0.get_located_child_owned(path) }
            pub fn get_located_children_owned<Joiner:#path_primitive,Child:Clone>(&#lt self) -> impl IntoIterator<Item = #owned_obj_at_path<Child,#path_pair<#path_type,Joiner>>> where #obj_type: #has_children<Joiner,Child>
                { self.0.get_located_children_owned() }
            pub fn into_located_children_owned<Joiner:#path_primitive,Child:Clone>(self) -> impl IntoIterator<Item = #owned_obj_at_path<Child,#path_pair<#path_type,Joiner>>> where #obj_type: #has_children<Joiner,Child> + Clone /*, AtPath:#lt */
                { self.0.into_located_children_owned() }
        }
        // Implement HasDescendants
        impl <#lt,#generic_bounds> #unowned_struct<#lt,#generics> {
            pub fn valid_paths<Joiner:#path,Descendant,J>(&#lt self) -> impl IntoIterator<Item = Joiner> where #obj_type: #has_descendants<#lt,Joiner,J,Descendant>
                { self.0.valid_paths() }
            
            pub fn get_descendant<Joiner:#path,Descendant,J>(&#lt self, path: &Joiner) -> Result<&#lt Descendant,()> where #obj_type: #has_descendants<#lt,Joiner,J,Descendant>
                { self.0.get_descendant(path) }
            pub fn get_descendants<Joiner:#path,Descendant:#lt,J>(&#lt self) -> impl IntoIterator<Item = &#lt Descendant> where #obj_type: #has_descendants<#lt,Joiner,J,Descendant>
                { self.0.get_descendants() }

            pub fn get_descendant_owned<Joiner:#path,Descendant:Clone,J:Clone>(&#lt self, path: &Joiner) -> Result<Descendant,()> where #obj_type: #has_descendants<#lt,Joiner,J,Descendant>
                { self.0.get_descendant_owned(path) }
            pub fn get_descendants_owned<Joiner:#path,Descendant:Clone,J:Clone>(&#lt self) -> impl IntoIterator<Item = Descendant> where #obj_type: #has_descendants<#lt,Joiner,J,Descendant>
                { self.0.get_descendants_owned() }

            pub fn get_located_descendant<Joiner:#path,Descendant,J>(&#lt self, path: Joiner) -> Result<#obj_at_path<#lt,Descendant,#path_pair<#path_type,Joiner>>,()> where #obj_type: #has_descendants<#lt,Joiner,J,Descendant>
                { self.0.get_located_descendant(path) }
            pub fn get_located_descendants<Joiner:#path,Descendant:#lt,J>(&#lt self) -> impl IntoIterator<Item = #obj_at_path<#lt,Descendant,#path_pair<#path_type,Joiner>>> where #obj_type: #has_descendants<#lt,Joiner,J,Descendant>
                { self.0.get_located_descendants() }

            pub fn get_located_descendant_owned<Joiner:#path,Descendant:Clone,J:Clone>(&#lt self, path: Joiner) -> Result<#owned_obj_at_path<Descendant,#path_pair<#path_type,Joiner>>,()> where #obj_type: #has_descendants<#lt,Joiner,J,Descendant>
                { self.0.get_located_descendant_owned(path) }
            pub fn get_located_descendants_owned<Joiner:#path,Descendant:Clone,J:Clone>(&#lt self) -> impl IntoIterator<Item = #owned_obj_at_path<Descendant,#path_pair<#path_type,Joiner>>> where #obj_type: #has_descendants<#lt,Joiner,J,Descendant>
                { self.0.get_located_descendants_owned() }
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

            pub fn replace_path<NewPath: #path>(self, function: impl Fn(#path_type) -> NewPath) -> #owned_obj_at_path<#obj_type,NewPath>
                { self.0.replace_path(function) }
        }
        // Implement HasChildren
        impl <#generic_bounds> #owned_struct<#generics> {
            pub fn valid_primitive_paths<Joiner:#path_primitive,Child>(&self) -> impl IntoIterator<Item = Joiner> where #obj_type: #has_children<Joiner,Child>  
                { self.0.valid_primitive_paths() }
            
            pub fn get_child<Joiner:#path_primitive,Child>(&self, path: &Joiner) -> Result<&Child,()> where #obj_type: #has_children<Joiner,Child>
                { self.0.get_child(path) }
            pub fn get_children<#lt,Joiner:#path_primitive,Child:#lt>(&#lt self) -> impl IntoIterator<Item = &#lt Child> where #obj_type: #has_children<Joiner,Child>
                { self.0.get_children() }
            
            pub fn get_child_owned<Joiner:#path_primitive,Child:Clone>(&self, path: &Joiner) -> Result<Child,()> where #obj_type: #has_children<Joiner,Child>
                { self.0.get_child_owned(path) }
            pub fn get_children_owned<Joiner:#path_primitive,Child:Clone>(&self) -> impl IntoIterator<Item = Child> where #obj_type: #has_children<Joiner,Child>
                { self.0.get_children_owned() }

            pub fn get_located_child<#lt,Joiner:#path_primitive,Child>(&#lt self, path: Joiner) -> Result<#obj_at_path<#lt,Child,#path_pair<#path_type,Joiner>>,()> where #obj_type: #has_children<Joiner,Child>
                { self.0.get_located_child(path) }
            pub fn get_located_children<#lt,Joiner:#path_primitive,Child:#lt>(&#lt self) -> impl IntoIterator<Item = #obj_at_path<#lt,Child,#path_pair<#path_type,Joiner>>> where #obj_type: #has_children<Joiner,Child>
                { self.0.get_located_children() }

            pub fn get_located_child_owned<Joiner:#path_primitive,Child:Clone>(&self, path: Joiner) -> Result<#owned_obj_at_path<Child,#path_pair<#path_type,Joiner>>,()> where #obj_type: #has_children<Joiner,Child>
                { self.0.get_located_child_owned(path) }
            pub fn get_located_children_owned<#lt,Joiner:#path_primitive,Child:Clone>(&#lt self) -> impl IntoIterator<Item = #owned_obj_at_path<Child,#path_pair<#path_type,Joiner>>> where #obj_type: #has_children<Joiner,Child>
                { self.0.get_located_children_owned() }
            pub fn into_located_children_owned<Joiner:#path_primitive,Child:Clone>(self) -> impl IntoIterator<Item = #owned_obj_at_path<Child,#path_pair<#path_type,Joiner>>> where #obj_type: #has_children<Joiner,Child> + Clone /*, AtPath:#lt */
                { self.0.into_located_children_owned() }
        }
        // Implement HasDescendants
        impl <#generic_bounds> #owned_struct<#generics> {
            pub fn valid_paths<#lt,Joiner:#path,Descendant,J>(&#lt self) -> impl IntoIterator<Item = Joiner> where #obj_type: #has_descendants<#lt,Joiner,J,Descendant>
                { self.0.valid_paths() }
            
            pub fn get_descendant<#lt,Joiner:#path,Descendant,J>(&#lt self, path: &Joiner) -> Result<&#lt Descendant,()> where #obj_type: #has_descendants<#lt,Joiner,J,Descendant>
                { self.0.get_descendant(path) }
            pub fn get_descendants<#lt,Joiner:#path,Descendant:#lt,J>(&#lt self) -> impl IntoIterator<Item = &#lt Descendant> where #obj_type: #has_descendants<#lt,Joiner,J,Descendant>
                { self.0.get_descendants() }

            pub fn get_descendant_owned<#lt,Joiner:#path,Descendant:Clone,J:Clone>(&#lt self, path: &Joiner) -> Result<Descendant,()> where #obj_type: #has_descendants<#lt,Joiner,J,Descendant>
                { self.0.get_descendant_owned(path) }
            pub fn get_descendants_owned<#lt,Joiner:#path,Descendant:Clone,J:Clone>(&#lt self) -> impl IntoIterator<Item = Descendant> where #obj_type: #has_descendants<#lt,Joiner,J,Descendant>
                { self.0.get_descendants_owned() }

            pub fn get_located_descendant<#lt,Joiner:#path,Descendant,J>(&#lt self, path: Joiner) -> Result<#obj_at_path<#lt,Descendant,#path_pair<#path_type,Joiner>>,()> where #obj_type: #has_descendants<#lt,Joiner,J,Descendant>
                { self.0.get_located_descendant(path) }
            pub fn get_located_descendants<#lt,Joiner:#path,Descendant:#lt,J>(&#lt self) -> impl IntoIterator<Item = #obj_at_path<#lt,Descendant,#path_pair<#path_type,Joiner>>> where #obj_type: #has_descendants<#lt,Joiner,J,Descendant>
                { self.0.get_located_descendants() }

            pub fn get_located_descendant_owned<#lt,Joiner:#path,Descendant:Clone,J:Clone>(&#lt self, path: Joiner) -> Result<#owned_obj_at_path<Descendant,#path_pair<#path_type,Joiner>>,()> where #obj_type: #has_descendants<#lt,Joiner,J,Descendant>
                { self.0.get_located_descendant_owned(path) }
            pub fn get_located_descendants_owned<#lt,Joiner:#path,Descendant:Clone,J:Clone>(&#lt self) -> impl IntoIterator<Item = #owned_obj_at_path<Descendant,#path_pair<#path_type,Joiner>>> where #obj_type: #has_descendants<#lt,Joiner,J,Descendant>
                { self.0.get_located_descendants_owned() }
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
