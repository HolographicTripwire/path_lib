use proc_macro2::{Span, TokenTree};
use syn::{Path, Result as SynResult, Token, parse::{Parse, ParseStream}};

pub struct PathList {
    pub paths: Vec<Path>
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
