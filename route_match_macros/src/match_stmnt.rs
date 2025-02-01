use proc_macro2::TokenStream;
use quote::quote;
use quote::quote_spanned;
use syn::braced;
use syn::parenthesized;
use syn::parse::Parse;
use syn::spanned::Spanned;
use syn::token::Paren;
use syn::Expr;
use syn::Token;

use crate::route::Route;

#[derive(Debug)]
pub struct MatchStmnt {
    arg: MatchArg,
    arms: Vec<MatchArm>,
}

#[derive(Debug)]
pub enum MatchArg {
    RequestProvider(Expr),
    ComponentProviders {
        method_provider: Expr,
        path_provider: Expr,
    },
}

impl Parse for MatchArg {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.peek(Paren) {
            // If we're parsing a parenthetical arg,
            // we parse the match arg within the parens
            let content;
            parenthesized!(content in input);
            return content.parse();
        }
        let first: Expr = input.parse()?;
        if input.peek(Token![,]) {
            let _: Token![,] = input.parse()?;
            let second: Expr = input.parse()?;
            return Ok(Self::ComponentProviders {
                method_provider: first,
                path_provider: second,
            });
        }
        Ok(Self::RequestProvider(first))
    }
}

#[derive(Debug)]
pub enum MatchArm {
    Route(Route),
    Default(Expr),
}

impl Parse for MatchArm {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.peek(Token![_]) {
            let forked_input = input.fork();
            let _: Token![_] = forked_input.parse()?;
            if forked_input.peek(Token![=>]) {
                let _: Token![_] = input.parse()?;
                let _: Token![=>] = input.parse()?;
                let expr: Expr = input.parse()?;
                if input.peek(Token![,]) {
                    let _: Token![,] = input.parse()?;
                }
                return Ok(Self::Default(expr));
            }
            // parse Default arm
        }
        let route: Route = input.parse()?;
        if input.peek(Token![,]) {
            let _: Token![,] = input.parse()?;
        }
        Ok(Self::Route(route))
    }
}

impl Parse for MatchStmnt {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let _: Token![match] = input.parse()?;
        let arg: MatchArg = input.parse()?;
        let content;
        let mut arms: Vec<MatchArm> = vec![];
        braced!(content in input);
        while !content.is_empty() {
            arms.push(content.parse()?);
        }

        Ok(Self { arg, arms })
    }
}

impl MatchArg {
    pub fn generate(&self) -> TokenStream {
        let tokens = match self {
            MatchArg::RequestProvider(req) => Self::generate_request_args(req),
            MatchArg::ComponentProviders {
                method_provider: method,
                path_provider: components,
            } => Self::generate_component_args(method, components),
        };

        quote! {
            #tokens
        }
    }

    pub fn generate_request_args(_request_provider: &Expr) -> TokenStream {
        todo!()
        // let span = request_provider.span();
        // quote_spanned! { span =>
        //     let _method: &str = HttpMethodProvider::method_str(#request_provider);
        //     let _path_str: &str = UrlPathProvider::path_str(#request_provider);
        //     let _path: Vec<&str> = _path_str
        //         .clone()
        //         .split('/')
        //         .filter(|comp| !comp.is_empty())
        //         .collect();
        // }
    }

    pub fn generate_component_args(method_provider: &Expr, path_provider: &Expr) -> TokenStream {
        let span = method_provider.span();
        let method_decl = quote_spanned! { span =>
            let _method = #method_provider;
        };
        let span = path_provider.span();
        let path_decl = quote_spanned! { span =>
            let _path_str = #path_provider;
            let _path: Vec<&str> = _path_str
                .split('/')
                .filter(|comp| !comp.is_empty())
                .collect();
        };
        quote! {
            #method_decl
            #path_decl
        }
    }
}

impl MatchStmnt {
    pub fn generate(&self) -> TokenStream {
        let method_and_path = &self.arg.generate();
        let conditionals: Vec<TokenStream> = self
            .arms
            .iter()
            .map(|route| route.generate_conditional())
            .collect();

        quote! {
            #[allow(unused_parens)]
            {
                #method_and_path
                #(#conditionals)else*
            }
        }
    }
}

impl MatchArm {
    pub fn generate_conditional(&self) -> TokenStream {
        match self {
            MatchArm::Route(route) => route.generate_conditional(),
            MatchArm::Default(expr) => quote_spanned! { expr.span() =>
                {
                    #expr
                }
            },
        }
    }
}
