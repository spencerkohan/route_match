use proc_macro2::Span;
use syn::parse::discouraged::Speculative;
use syn::parse::Parse;
use syn::spanned::Spanned;
use syn::Ident;
use syn::Token;

#[derive(Debug)]
pub enum PathComponent {
    Ident(Ident),
    Param(Ident),
    Wildcard(Span),
    Rest(Span, Option<Ident>),
    Any(Span),
}

impl Parse for PathComponent {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.peek(Token![:]) {
            // this is a param
            let _: Token![:] = input.parse()?;
            let name: Ident = input.parse()?;
            Ok(PathComponent::Param(name))
        } else if input.peek(Token![_]) {
            // this is a param
            let token: Token![_] = input.parse()?;
            Ok(PathComponent::Any(token.span))
        } else if input.peek(Token![..]) {
            let elipsis: Token![..] = input.parse()?;
            if input.peek(Token![:]) {
                let _: Token![:] = input.parse()?;
                let name: Ident = input.parse()?;
                let span = elipsis.span();
                Ok(PathComponent::Rest(span, Some(name)))
            } else {
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

        if input.peek(Token![_]) {
            components.push(input.parse()?);
            if input.peek(Token![=>]) {
                return Ok(Self { components });
            }
            let _: Token![=>] = input.parse()?;
        }

        loop {
            if input.peek(Token![=>]) {
                break;
            }

            if input.peek(Token![..]) {
                components.push(input.parse()?);
                if input.peek(Token![=>]) {
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
            PathComponent::Any(span) => span.clone(),
        }
    }
}
