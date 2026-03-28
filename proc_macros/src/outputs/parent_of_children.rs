use proc_macro2::TokenStream;

use quote::quote;
use syn::{LitStr, TypePath};
use crate::PathlibImports;
use crate::inputs::function_names::{PluralSnakecaseIdents, SingularSnakecaseIdents, PluralTitlecaseIdents};
use crate::utils::types::type_with_bounds::TypeWithBounds;

pub fn parent_of_children_toks(bounded_child_type: &TypeWithBounds, path_type: &TypePath, singular_snakecase: &LitStr, plural_snakecase: &LitStr, plural_titlecase: &LitStr) -> TokenStream {
    // Generics
    let self_bounds = bounded_child_type.self_bounds_toks();
    let child_generic_bounds = bounded_child_type.nonself_bounds_toks();
    // Types
    let child_type = &bounded_child_type.bounded_ty;
    let lt = quote! {'a};
    // Imports from path_lib
    let imports = PathlibImports::default();
    let obj_at_path  = imports.obj_at_path;
    let owned_obj_at_path = imports.owned_obj_at_path;
    // Get ident names
    let singular_snakecase_idents = SingularSnakecaseIdents::new(singular_snakecase);
    let plural_snakecase_idents = PluralSnakecaseIdents::new(plural_snakecase);
    let plural_titlecase_idents = PluralTitlecaseIdents::new(plural_titlecase);
    let parent_of_children = plural_titlecase_idents.parent_of_children;
    
    let get_child_paths = singular_snakecase_idents.get_child_paths;
    let get_child = singular_snakecase_idents.get_child;
    let get_children = plural_snakecase_idents.get_children;
    let get_child_owned = singular_snakecase_idents.get_child_owned;
    let get_children_owned = plural_snakecase_idents.get_children_owned;
    
    let get_located_child = singular_snakecase_idents.get_located_child;
    let get_located_children = plural_snakecase_idents.get_located_children;
    let get_located_child_owned = singular_snakecase_idents.get_located_child_owned;
    let get_located_children_owned = plural_snakecase_idents.get_located_children_owned;
    let into_located_children_owned = plural_snakecase_idents.into_located_children_owned;

    // Build final tokens:
    quote! {
        pub trait #parent_of_children<#child_generic_bounds>: #self_bounds {
            fn #get_child_paths(&self) -> impl IntoIterator<Item = #path_type>;
            fn #get_child(&self, path: &#path_type) -> Result<&#child_type,()>;
            fn #get_children<#lt>(&#lt self) -> impl IntoIterator<Item = &#lt #child_type> where #child_type: #lt {
                self.#get_child_paths()
                    .into_iter()
                    .map(|path| self.#get_child(&path).expect("#get_child_paths returned an invalid path"))
            }

            fn #get_child_owned(&self, path: &#path_type) -> Result<#child_type,()> where #child_type: Clone
                { self.#get_child(path).cloned() }
            fn #get_children_owned(&self) -> impl IntoIterator<Item = #child_type> where #child_type: Clone {
                self.#get_child_paths()
                    .into_iter()
                    .map(|path| self.#get_child_owned(&path).expect("#get_child_paths returned an invalid path"))
            }

            fn #get_located_child<#lt>(&#lt self, path: #path_type) -> Result<#obj_at_path<#lt,#child_type,#path_type>,()>
                { Ok(#obj_at_path{obj: self.#get_child(&path)?, path}) }
            fn #get_located_children<#lt>(&#lt self) -> impl IntoIterator<Item = #obj_at_path<'a,#child_type,#path_type>> where #child_type: #lt {
                self.#get_child_paths()
                    .into_iter()
                    .map(|path| { self.#get_located_child(path).expect("#get_child_paths returned an invalid path")})
            }
            
            fn #get_located_child_owned(&self, path: #path_type) -> Result<#owned_obj_at_path<#child_type,#path_type>,()> where #child_type: Clone
                { Ok(#owned_obj_at_path{obj: self.#get_child_owned(&path)?,path}) }
            fn #get_located_children_owned(&self) -> impl IntoIterator<Item = #owned_obj_at_path<#child_type,#path_type>> where #child_type: Clone {
                self.#get_child_paths()
                    .into_iter()
                    .map(|path| { self.#get_located_child_owned(path).expect("#get_child_paths returned an invalid path")})
            }
            fn #into_located_children_owned(self) -> impl IntoIterator<Item = #owned_obj_at_path<#child_type,#path_type>> where #child_type: Clone, Self: Sized
                { self.#get_located_children_owned().into_iter().collect::<Vec<_>>() }
        }
        
    }
}
