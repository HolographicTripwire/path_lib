use std::fmt::Display;

use proc_macro2::TokenStream;
use syn::{Lifetime, PredicateLifetime, PredicateType, Token, Type, TypeParamBound, WhereClause, WherePredicate, parse::{Parse, ParseStream}, token::Plus};
use quote::quote;
use syn::{Result as SynResult};

use crate::utils::types::{check_selftype, quote_vec};

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

    pub fn lifetime_generics(&self) -> impl IntoIterator<Item=Lifetime> {
        self.pred_lifetimes()
            .into_iter()
            .map(|lifetime| lifetime.lifetime)
    }
    pub fn lifetime_generics_toks(&self) -> TokenStream {
        let lifetimes: Vec<_> = self.lifetime_generics().into_iter().collect();
        let extra_comma = if lifetimes.len() > 0 { quote!{,} } else { quote!{} };
        quote!{#(#lifetimes),* #extra_comma }
    }
    pub fn type_generics(&self) -> impl IntoIterator<Item=Type> {
        self.pred_types()
            .into_iter()
            .map(|pred_type| pred_type.bounded_ty )
    }
    pub fn nonself_type_generics(&self) -> impl IntoIterator<Item=Type> {
        self.type_generics()
            .into_iter()
            .filter(|x| !check_selftype(x))
    }
    pub fn nonself_type_generics_toks(&self) -> TokenStream {
        let types: Vec<_> = self.nonself_type_generics().into_iter().collect();
        let extra_comma = if types.len() > 0 { quote!{,} } else { quote!{} };
        quote!{#(#types),* #extra_comma }
    }

    // pub fn generic_bounds(&self) -> TokenStream {
    //     match &self.where_clause {
    //         Some(where_clause) => {
    //             let predicates = &where_clause.predicates;
    //             quote! { #predicates }
    //         }, None => quote! {},
    //     }
    // }
    pub fn self_bounds(&self) -> Vec<syn::punctuated::Punctuated<TypeParamBound,Plus>> {
        if let Some(where_clause) = &self.where_clause {
            where_clause.predicates.iter().filter_map(
                |x| match x {
                    WherePredicate::Lifetime(_) => None,
                    WherePredicate::Type(t) => if check_selftype(&t.bounded_ty)
                        { Some(t.bounds.clone()) } else { None },
                    _ => todo!(),
                }).collect()
        } else { vec![] }
    }
    pub fn self_bounds_toks(&self) -> TokenStream {
        let bounds = self.self_bounds();
        quote!{ #(#bounds)+*  }
    }
    pub fn nonself_bounds(&self) -> Vec<WherePredicate> {
        if let Some(where_clause) = &self.where_clause {
            where_clause.predicates.iter().filter_map(
                |x| match x {
                    WherePredicate::Lifetime(l) => Some(WherePredicate::Lifetime(l.clone())),
                    WherePredicate::Type(t) => if check_selftype(&t.bounded_ty)
                        { None } else { Some(WherePredicate::Type(t.clone())) },
                    _ => todo!(),
                }).collect()
            
        } else { vec![] }
    }
    pub fn nonself_bounds_toks(&self) -> TokenStream {
        let bounds = self.nonself_bounds();
        let extra_comma = if bounds.len() > 0 { quote!{,} } else { quote!{} };
        quote!{ #(#bounds),* #extra_comma }
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
