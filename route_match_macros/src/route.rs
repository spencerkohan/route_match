use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::quote;
use quote::quote_spanned;
use syn::parse::Parse;
use syn::spanned::Spanned;
use syn::Expr;
use syn::Ident;
use syn::LitStr;
use syn::Token;

use crate::route_impl::Path;
use crate::route_impl::PathComponent;

#[derive(Debug)]
pub struct Route {
    pub method: Ident,
    pub path: Path,
    pub expr: Expr,
}

impl Parse for Route {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let method: Ident = input.parse()?;
        let path: Path = input.parse()?;
        let _: Token![=>] = input.parse()?;
        let expr: Expr = input.parse()?;
        Ok(Route { method, path, expr })
    }
}

impl Route {
    pub fn span(&self) -> Span {
        self.method
            .span()
            .join(self.expr.span())
            .unwrap_or(self.expr.span().clone())
    }

    pub fn generate_conditional(&self) -> TokenStream {
        let args = self.args();
        let method = &self.method;
        let method_str = LitStr::new(&method.to_string(), method.span());
        let expr = &self.expr;
        let expr = quote_spanned! { expr.span() =>
            #expr
        };
        let arg_assignments = self.arg_assignments();
        let match_conditions = self.match_conditions();

        let method_condition = quote_spanned! { method.span() =>
            if _method != &#method_str {
                // If the method doesn't match, return None
                None
            }
        };
        let path_conditions = quote_spanned! { self.path.span() =>
            #match_conditions
            else {
                // Otherwise we assign the args and return them
                #arg_assignments
                Some((#args))
            }
        };

        quote_spanned! { self.span() =>
            if let Some((#args)) = {
                #method_condition
                #path_conditions
            } {
                #expr
            }
        }
    }

    fn match_conditions(&self) -> TokenStream {
        if self.has_wildcard() {
            quote! {}
        } else {
            let static_conditions = self.static_conditions();
            let count = self.path.components.len();
            quote_spanned! { self.path.span() =>
                else if _path.len() != #count {
                    // If the path comoponent doesn't match, return None
                    None
                } else if #static_conditions {
                    // If any of the satic conditions don't match, return None
                    None
                }
            }
        }
    }

    fn has_wildcard(&self) -> bool {
        return (&self).path.components.iter().fold(false, |acc, cmp| {
            if let PathComponent::Wildcard(_) = cmp {
                true
            } else {
                acc
            }
        });
    }

    fn args(&self) -> TokenStream {
        let args: Vec<Ident> = (&self)
            .path
            .components
            .iter()
            .filter_map(|component| {
                if let PathComponent::Param(param) = component {
                    Some(param.clone())
                } else {
                    None
                }
            })
            .collect();

        if args.len() == 0 {
            return quote! {
                ()
            };
        }

        quote_spanned! { self.path.span() =>
            #(#args),*
        }
    }

    pub fn static_conditions(&self) -> TokenStream {
        let mut static_conditions: Vec<TokenStream> = vec![];

        for i in 0..self.path.components.len() {
            match &self.path.components[i] {
                PathComponent::Ident(name) => {
                    static_conditions.push(quote_spanned! { name.span() =>
                        _path[#i] != stringify!(#name)
                    });
                }
                PathComponent::Param(_) => {}
                PathComponent::Wildcard(_) => {}
            }
        }

        quote! {
            #(#static_conditions)||*
        }
    }

    pub fn arg_assignments(&self) -> TokenStream {
        let mut assignments: Vec<TokenStream> = vec![];

        for i in 0..self.path.components.len() {
            match &self.path.components[i] {
                PathComponent::Ident(_) => {}
                PathComponent::Param(name) => {
                    assignments.push(quote_spanned! { name.span() =>
                        let #name = _path[#i];
                    });
                }
                PathComponent::Wildcard(_) => {}
            }
        }

        quote! {
            #(#assignments)*
        }
    }
}
