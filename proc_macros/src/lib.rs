mod utils;
mod inputs;
mod outputs;
mod macros;

use crate::utils::imports::import;
use proc_macro2::TokenStream;
use syn::{Error, Result as SynResult};

struct PathlibImports {
    path: TokenStream,
    path_primitive: TokenStream,
    path_pair: TokenStream,
    
    obj_at_path: TokenStream,
    owned_obj_at_path: TokenStream,
    
    has_children: TokenStream,
    has_descendants: TokenStream,
}
impl Default for PathlibImports {
    fn default() -> Self {
        let crate_id = "path_lib";
        Self {
            path: import(crate_id, "paths::Path"),
            path_primitive: import(crate_id, "paths::PathPrimitive"),
            path_pair: import(crate_id, "paths::PathPair"),
            
            obj_at_path: import(crate_id, "obj_at_path::ObjAtPath"),
            owned_obj_at_path: import(crate_id, "obj_at_path::OwnedObjAtPath"),
            
            has_children: import(crate_id, "HasChildren"),
            has_descendants: import(crate_id, "HasDescendants"),
        }
    }
}
fn transform_syn_err<T, F: FnOnce(String) -> String>(r: SynResult<T>, f: F) -> SynResult<T> {
    r.map_err(|e| Error::new(e.span(), f(e.to_string())))
}


#[proc_macro]
pub fn generate_obj_at_path_wrappers(input: proc_macro::TokenStream) -> proc_macro::TokenStream
    { macros::generate_obj_at_path_wrappers(input) }
#[proc_macro]
pub fn generate_parent_of_children_trait(input: proc_macro::TokenStream) -> proc_macro::TokenStream
    { macros::generate_parent_of_children_trait(input) }
