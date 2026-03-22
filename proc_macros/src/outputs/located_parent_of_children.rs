use proc_macro2::TokenStream;

use quote::quote;
use syn::{LitStr, TypePath};
use crate::PathlibImports;
use crate::inputs::function_names::{PluralSnakecaseIdents, SingularSnakecaseIdents, PluralTitlecaseIdents};
use crate::utils::types::type_with_bounds::TypeWithBounds;

pub fn located_parent_of_children_toks(bounded_child_type: &TypeWithBounds, path_type: &TypePath, singular_snakecase: &LitStr, plural_snakecase: &LitStr, plural_titlecase: &LitStr) -> TokenStream {
    // Generics
    let child_generics = bounded_child_type.generics();
    let child_generic_bounds = bounded_child_type.generic_bounds();
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
    let located_parent_of_children = plural_titlecase_idents.located_parent_of_children;
    
    let get_child_paths = singular_snakecase_idents.get_child_paths;
    let get_joined_child_paths = singular_snakecase_idents.get_joined_child_paths;
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
        pub trait #located_parent_of_children<#lt,Parent,ParentPath,JoinedPath:From<(ParentPath,#path_type)>,#child_generic_bounds>
            where Parent: #parent_of_children<#child_generics> + #lt, ParentPath: #lt + Clone {
            fn self_ref(&self) -> &#obj_at_path<#lt,Parent,ParentPath>;
            fn self_owned(self) -> #obj_at_path<#lt,Parent,ParentPath>;
            fn #get_child_paths(&self) -> impl IntoIterator<Item = #path_type>
                { self.self_ref().obj.#get_child_paths() }
            fn #get_joined_child_paths(&self) -> impl IntoIterator<Item = JoinedPath> {
                self.#get_child_paths()
                    .into_iter()
                    .map(|path| self.self_ref().append_path(path))
            }
            fn #get_child(&#lt self, path: &#path_type) -> Result<&#child_type,()> { self.self_ref().obj.#get_child(path) }
            fn #get_children(&#lt self) -> impl IntoIterator<Item = &#lt #child_type> { self.self_ref().obj.#get_children() }

            fn #get_child_owned(&self, path: &#path_type) -> Result<#child_type,()> where #child_type: Clone { self.self_ref().obj.#get_child_owned(path) }
            fn #get_children_owned(&self) -> impl IntoIterator<Item = #child_type> where #child_type: Clone { self.self_ref().obj.#get_children_owned() }

            fn #get_located_child(&self, path: #path_type) -> Result<#obj_at_path<#lt,#child_type,JoinedPath>,()> { 
                let self_ref = self.self_ref();
                Ok(self_ref.obj.#get_located_child(path)?
                    .prepend_path_to_self(self_ref.path.clone()))
            }
            fn #get_located_children(&#lt self) -> impl IntoIterator<Item = #obj_at_path<'a,#child_type,JoinedPath>> {
                let self_ref = self.self_ref();
                self_ref.obj.#get_located_children()
                    .into_iter()
                    .map(|inner| inner.prepend_path_to_self(self_ref.path.clone()))
            }
            
            fn #get_located_child_owned(&self, path: #path_type) -> Result<#owned_obj_at_path<#child_type,JoinedPath>,()> where #child_type: Clone {
                let self_ref = self.self_ref();
                Ok(self_ref.obj.#get_located_child_owned(path)?
                    .prepend_path_to_self(self_ref.path.clone()))
            }
            fn #get_located_children_owned(&#lt self) -> impl IntoIterator<Item = #owned_obj_at_path<#child_type,JoinedPath>> where #child_type: Clone {
                let self_ref = self.self_ref();
                self_ref.obj.#get_located_children_owned()
                    .into_iter()
                    .map(|inner| inner.prepend_path_to_self(self_ref.path.clone()))
            }
            // fn #into_located_children(self) -> impl IntoIterator<Item = #owned_obj_at_path<#child_type,JoinedPath>> where #child_type: Clone, Self: Sized {
            //     let self_owned = self.self_owned()
            //     self_owned
            // }
        }

        impl <#lt,Parent,ParentPath,JoinedPath,#child_generic_bounds> #located_parent_of_children<#lt,Parent,ParentPath,JoinedPath,#child_generics>
        for #obj_at_path<#lt,Parent,ParentPath> where Parent:#parent_of_children<#child_generics>, ParentPath: #lt + Clone, JoinedPath: From<(ParentPath,#path_type)> {
            fn self_ref(&self) -> &#obj_at_path<#lt,Parent,ParentPath> { self }
            fn self_owned(self) -> #obj_at_path<#lt,Parent,ParentPath> { self }
        }
    }
}
