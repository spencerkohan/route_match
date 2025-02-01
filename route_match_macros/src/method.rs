use proc_macro2::Span;
use syn::parse::Parse;
use syn::spanned::Spanned;
use syn::Ident;
use syn::Token;

#[derive(Debug)]
pub enum Method {
    Any(Span),
    Some(Ident),
}

impl Parse for Method {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.peek(Token![_]) {
            let token: Token![_] = input.parse()?;
            return Ok(Method::Any(token.span()));
        }
        let ident: Ident = input.parse()?;
        Ok(Method::Some(ident))
    }
}

impl Method {
    pub fn span(&self) -> Span {
        match self {
            Self::Any(span) => span.clone(),
            Self::Some(method) => method.span().clone(),
        }
    }
}
