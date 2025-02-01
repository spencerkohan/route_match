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

use crate::method::Method;
use crate::route_impl::Path;
use crate::route_impl::PathComponent;

#[derive(Debug)]
pub struct Route {
    pub method: Method,
    pub path: Path,
    pub expr: Expr,
}

impl Parse for Route {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let method: Method = input.parse()?;
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

    pub fn method_condition(&self) -> TokenStream {
        let method = &self.method;
        match method {
            Method::Any(_) => quote! {},
            Method::Named(method) => {
                let method_str = LitStr::new(&method.to_string(), method.span());
                let method_condition = quote_spanned! { method.span() =>
                    if _method != &#method_str {
                        // If the method doesn't match, return None
                        None
                    }
                };
                method_condition
            }
            Method::Param(_) => quote! {},
        }
    }

    pub fn generate_conditional(&self) -> TokenStream {
        let args = self.args();

        let expr = &self.expr;
        let expr = quote_spanned! { expr.span() =>
            #expr
        };
        let arg_assignments = self.arg_assignments();
        let match_conditions = self.match_conditions();

        let method_condition = self.method_condition();

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
        let leading_token = match self.method {
            Method::Any(_) => quote! {},
            Method::Param(_) => quote! {},
            Method::Named(_) => quote! { else },
        };

        if self.has_indeterminate_length() {
            let static_conditions = self.static_conditions();
            quote_spanned! { self.path.span() =>
                #leading_token if #static_conditions {
                    // If any of the satic conditions don't match, return None
                    None
                }
            }
        } else {
            let static_conditions = self.static_conditions();
            let count = self.path.components.len();
            quote_spanned! { self.path.span() =>
                #leading_token if _path.len() != #count {
                    // If the path comoponent doesn't match, return None
                    None
                } else if #static_conditions {
                    // If any of the satic conditions don't match, return None
                    None
                }
            }
        }
    }

    fn has_indeterminate_length(&self) -> bool {
        return (&self).path.components.iter().fold(false, |acc, cmp| {
            if let PathComponent::Wildcard(_) = cmp {
                true
            } else if let PathComponent::Rest(_, _) = cmp {
                true
            } else {
                acc
            }
        });
    }

    fn args(&self) -> TokenStream {
        let mut args: Vec<Ident> = (&self)
            .path
            .components
            .iter()
            .filter_map(|component| {
                if let PathComponent::Param(param) = component {
                    Some(param.clone())
                } else if let PathComponent::Rest(_, Some(param)) = component {
                    Some(param.clone())
                } else {
                    None
                }
            })
            .collect();

        if let Method::Param(param) = &self.method {
            args.push(param.clone());
        }

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
                PathComponent::Rest(_, _) => {}
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
                PathComponent::Rest(_, Some(name)) => {
                    let assignment = quote_spanned! { name.span() =>
                        let mut byte_offset = 0;
                        let mut segment_count = 0;

                        for (idx, c) in _path_str.char_indices() {
                            if c == '/' {
                                if segment_count == #i {
                                    byte_offset = idx + 1; // Skip past the '/'
                                    break;
                                }
                                segment_count += 1;
                            }
                        }
                        let #name = &_path_str[byte_offset..];
                    };
                    eprintln!("Added assignment: {}", assignment);
                    assignments.push(assignment);
                }
                PathComponent::Rest(_, _) => {}
                PathComponent::Wildcard(_) => {}
            }
        }

        if let Method::Param(name) = &self.method {
            assignments.push(quote! {
                let #name = _method;
            });
        }

        quote! {
            #(#assignments)*
        }
    }
}
