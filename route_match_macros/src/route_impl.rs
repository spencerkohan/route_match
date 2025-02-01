use proc_macro2::TokenStream;

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
