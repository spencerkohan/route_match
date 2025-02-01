use proc_macro2::Span;
use syn::parse::Parse;
use syn::spanned::Spanned;
use syn::Ident;
use syn::Token;

#[derive(Debug)]
pub enum Method {
    Any(Span),
    Named(Ident),
    Param(Ident),
}

impl Parse for Method {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.peek(Token![_]) {
            let token: Token![_] = input.parse()?;
            return Ok(Method::Any(token.span()));
        } else if input.peek(Token![:]) {
            let _: Token![:] = input.parse()?;
            let ident: Ident = input.parse()?;
            return Ok(Method::Param(ident));
        }
        let ident: Ident = input.parse()?;
        Ok(Method::Named(ident))
    }
}

impl Method {
    pub fn span(&self) -> Span {
        match self {
            Self::Any(span) => span.clone(),
            Self::Named(method) => method.span().clone(),
            Self::Param(method) => method.span().clone(),
        }
    }
}
