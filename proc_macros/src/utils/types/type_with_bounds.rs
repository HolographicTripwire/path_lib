use std::fmt::Display;

use proc_macro2::TokenStream;
use syn::{Lifetime, PredicateLifetime, PredicateType, Token, Type, WhereClause, WherePredicate, parse::{Parse, ParseStream}};
use quote::quote;
use syn::{Result as SynResult};

#[derive(Debug)]
pub struct TypeWithBounds {
    pub bounded_ty: Type,
    pub where_clause: Option<WhereClause>
}

impl TypeWithBounds {
    pub fn pred_lifetimes(&self) -> Vec<PredicateLifetime> {
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
    pub fn pred_types(&self) -> Vec<PredicateType> {
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

    pub fn generics(&self) -> TokenStream {
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

    pub fn generic_bounds(&self) -> TokenStream {
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
