use proc_macro2::Span;
use proc_macro2::TokenStream;
use syn::parse::discouraged::Speculative;
use syn::parse::Parse;
use syn::spanned::Spanned;
use syn::Ident;
use syn::Token;

use crate::match_stmnt::MatchStmnt;

pub fn parse(input: TokenStream) -> TokenStream {
    let stmnt: MatchStmnt = match syn::parse2::<MatchStmnt>(input) {
        Ok(stmnt) => stmnt,
        Err(err) => {
            return proc_macro2::TokenStream::from(err.to_compile_error());
        }
    };

    stmnt.generate()
}

#[derive(Debug)]
pub enum PathComponent {
    Ident(Ident),
    Param(Ident),
    Wildcard(Span),
    Rest(Span, Option<Ident>),
}

impl Parse for PathComponent {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.peek(Token![:]) {
            // this is a param
            let _: Token![:] = input.parse()?;
            let name: Ident = input.parse()?;
            Ok(PathComponent::Param(name))
        } else if input.peek(Token![..]) {
            eprintln!("parsing rest...");
            let elipsis: Token![..] = input.parse()?;
            if input.peek(Token![:]) {
                let _: Token![:] = input.parse()?;
                let name: Ident = input.parse()?;
                let span = elipsis.span();
                eprintln!("got name: {:?}", name);
                Ok(PathComponent::Rest(span, Some(name)))
            } else {
                eprintln!("got no name");
                let span = elipsis.span();
                Ok(PathComponent::Rest(span, None))
            }
        } else if input.peek(Token![*]) {
            let token: Token![:] = input.parse()?;
            Ok(PathComponent::Wildcard(token.span()))
        } else {
            let name: Ident = input.parse()?;
            Ok(PathComponent::Ident(name))
        }
    }
}

#[derive(Debug)]
pub struct Path {
    pub components: Vec<PathComponent>,
}

impl Parse for Path {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut components: Vec<PathComponent> = vec![];

        loop {
            if input.peek(Token![=>]) {
                break;
            }
            if input.peek(Token![..]) {
                components.push(input.parse()?);
                eprintln!("got elipsis");
                if input.peek(Token![=>]) {
                    eprintln!("end");
                    break;
                } else {
                    // A "rest" token should always be the final one
                    let _: Token![=>] = input.parse()?;
                }
            }
            let _: Token![/] = input.parse()?;
            let forked_input = input.fork();
            if let Ok(component) = forked_input.parse::<PathComponent>() {
                components.push(component);
                input.advance_to(&forked_input);
            }
        }

        Ok(Self { components })
    }
}

impl Path {
    pub fn span(&self) -> Span {
        let Some(first) = self.components.first() else {
            return Span::call_site();
        };
        let Some(last) = self.components.first() else {
            return first.span();
        };
        first.span().join(last.span()).unwrap_or(first.span())
    }
}

impl PathComponent {
    pub fn span(&self) -> Span {
        match self {
            PathComponent::Ident(ident) => ident.span(),
            PathComponent::Param(param) => param.span(),
            PathComponent::Wildcard(span) => span.clone(),
            PathComponent::Rest(span, _) => span.clone(),
        }
    }
}
