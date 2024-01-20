use quote::quote;
use syn::{parse_macro_input, DeriveInput};

pub(super) fn parse(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;
    let generics = input.generics;

    let options = quote! {
        impl #generics crate::cmd::args::WithOpts for #name #generics {
            fn with_opts(self, cmd: crate::Command) -> crate::Command {
                cmd.with_opts(self)
            }
        }
    };

    options.into()
}
