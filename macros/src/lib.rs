extern crate proc_macro;

mod create_cmd;
mod func;
mod options_builder;
mod with_options;

use create_cmd::CreateCommand;
use func::Func;
use proc_macro::TokenStream;
use syn::parse_macro_input;

#[proc_macro]
pub fn func(input: TokenStream) -> TokenStream {
    Func::new(input.into()).process().into()
}

#[proc_macro_derive(OptionsBuilder)]
pub fn options_builder(input: TokenStream) -> TokenStream {
    options_builder::parse(input)
}

#[proc_macro_derive(WithOpts)]
pub fn with_opts(input: TokenStream) -> TokenStream {
    with_options::parse(input)
}

#[proc_macro]
pub fn create_cmd(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as CreateCommand);
    input.build().into()
}
