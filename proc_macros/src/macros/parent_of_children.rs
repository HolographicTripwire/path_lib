use proc_macro::TokenStream;
use syn::parse::{Parse, ParseStream};
use syn::token::{Comma, Paren};
use syn::{LitStr, Result as SynResult, TypePath, parse_macro_input};
use syn::parenthesized;

use quote::quote;
use crate::outputs::located_parent_of_children::located_parent_of_children_toks;
use crate::outputs::owned_located_parent_of_children::owned_located_parent_of_children_toks;
use crate::outputs::parent_of_children::parent_of_children_toks;
use crate::{transform_syn_err};
use crate::utils::types::type_with_bounds::TypeWithBounds;

struct MacroInput {
    _child_type_paren: Paren,
    bounded_child_type: TypeWithBounds,
    _child_type_comma: Comma,

    path_type: TypePath,
    _path_type_comma: Comma,

    singular_snakecase: LitStr,
    _singular_snakecase_comma: Comma,
    plural_snakecase: LitStr,
    _plural_snakecase_comma: Comma,
    plural_titlecase: LitStr,
    // _plural_titlecase_comma: Comma,


    // unowned_struct_name: Ident,
    // _unowned_struct_name_comma: Comma,
    // unowned_struct_derives: PathList,
    // _unowned_struct_derives_comma: Comma,

    // owned_struct_name: Ident,
    // _owned_struct_name_comma: Comma,
    // owned_struct_derives: PathList,
    // _owned_struct_derives_comma: Comma,
}

impl Parse for MacroInput {
    fn parse(input: ParseStream) -> SynResult<Self> {
        // Parse first item: either a type (syn::Type) or a generic param (e.g. T: Clone)
        let content;
        Ok(MacroInput {
            _child_type_paren:
                parenthesized!(
                    content in input
                ),
            bounded_child_type:
                transform_syn_err(
                    content.parse(), |msg|
                    format!{"Error while parsing bounded_child_type (arg 1): {}", msg}
                )?,
            _child_type_comma:
                transform_syn_err(
                    input.parse(), |_msg|
                    format!{"Error while parsing bounded_child_type (arg 1): Missing comma after argument"}
                )?,
            
            path_type:
                transform_syn_err(
                    input.parse(),
                    |msg| format!{"Error while parsing path_type (arg 2): {}", msg}
                )?,
            _path_type_comma:
                transform_syn_err(
                    input.parse(), |_msg|
                    format!{"Error while parsing path_type (arg 2): Missing comma after argument"}
                )?,
            
            singular_snakecase:
                transform_syn_err(
                    input.parse(), |msg|
                    format!{"Error while parsing owned_name (arg 5): {}", msg}
                )?,
            _singular_snakecase_comma: 
                transform_syn_err(
                    input.parse(), |_msg|
                    format!{"Error while parsing path_type (arg 2): Missing comma after argument"}
                )?,
            plural_snakecase:
                transform_syn_err(
                    input.parse(), |msg|
                    format!{"Error while parsing owned_name (arg 5): {}", msg}
                )?,
            _plural_snakecase_comma:
                transform_syn_err(
                    input.parse(),
                    |_msg| format!{"Error while parsing path_type (arg 2): Missing comma after argument"}
                )?,
            plural_titlecase:
                transform_syn_err(
                    input.parse(), |msg|
                    format!{"Error while parsing owned_name (arg 5): {}", msg}
                )?,
            // _plural_titlecase_comma:
            //     transform_syn_err(
            //         input.parse(),
            //         |_msg| format!{"Error while parsing path_type (arg 2): Missing comma after argument"}
            //     )?,
            

            // unowned_struct_name: 
            //     ident_from_literal(transform_syn_err(
            //         input.parse(), |msg|
            //         format!{"Error while parsing unowned_struct_name (arg 3): {}", msg})?
            //     ),
            // _unowned_struct_name_comma:
            //     transform_syn_err(
            //         input.parse(), |_msg|
            //         format!{"Error while parsing unowned_struct_name (arg 3): Missing comma after argument"}
            //     )?,
            // unowned_struct_derives:
            //     transform_syn_err(
            //         input.parse(), |msg|
            //         format!{"Error while parsing unowned_struct_derives (arg 4): {}", msg}
            //     )?,
            // _unowned_struct_derives_comma:
            //     transform_syn_err(
            //         input.parse(), |_msg|
            //         format!{"Error while parsing unowned_struct_derives (arg 4): Missing comma after argument"}
            //     )?,

            // owned_struct_name:
            //     ident_from_literal(transform_syn_err(
            //         input.parse(), |msg|
            //         format!{"Error while parsing owned_struct_name (arg 5): {}", msg})?
            //     ),
            // _owned_struct_name_comma:
            //     transform_syn_err(
            //         input.parse(), |_msg|
            //         format!{"Error while parsing owned_struct_name (arg 5): Missing comma after argument"}
            //     )?,
            // owned_struct_derives:
            //     transform_syn_err(
            //         input.parse(), |msg|
            //         format!{"Error while parsing owned_struct_derives (arg 6): {}", msg}
            //     )?,
            // _owned_struct_derives_comma:
            //     transform_syn_err(
            //         input.parse(), |_msg|
            //         format!{"Error while parsing owned_struct_derives (arg 6): Missing comma after argument"}
            //     )?,
        })
    }
}

pub fn generate_parent_of_children_trait(input: proc_macro::TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as MacroInput);
    let parent_of_children_toks = parent_of_children_toks(
        &input.bounded_child_type,
        &input.path_type,
        &input.singular_snakecase,
        &input.plural_snakecase,
        &input.plural_titlecase,
    );
    let located_parent_of_children_toks = located_parent_of_children_toks(
        &input.bounded_child_type,
        &input.path_type,
        &input.singular_snakecase,
        &input.plural_snakecase,
        &input.plural_titlecase,
    );
    let owned_located_parent_of_children_toks = owned_located_parent_of_children_toks(
        &input.bounded_child_type,
        &input.path_type,
        &input.singular_snakecase,
        &input.plural_snakecase,
        &input.plural_titlecase,
    );
    let expanded = proc_macro::TokenStream::from(quote!{
        #parent_of_children_toks
        #located_parent_of_children_toks
        #owned_located_parent_of_children_toks
    });
    //panic!("{}", TokenStream::from(expanded).to_string());
    expanded
}
