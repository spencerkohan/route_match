use proc_macro::TokenStream;
mod match_stmnt;
mod method;
mod route;
mod route_impl;

///
#[proc_macro]
pub fn route(input: TokenStream) -> TokenStream {
    route_impl::parse(input.into()).into()
}
