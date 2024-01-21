use convert_case::{Case, Casing};
use proc_macro2::{TokenStream, TokenTree};
use quote::{quote, ToTokens};
use syn::{
    parenthesized,
    parse::{Parse, ParseStream},
    token::Paren,
    Ident, PathSegment, Token,
};

#[derive(Debug, Default)]
pub(super) struct CreateCommand {
    docs: Vec<TokenTree>,
    variants: Vec<Variant>,
}

#[derive(Debug)]
struct Variant {
    flags: Flags,
    name: Ident,
    term_type: Ident,
    args: Vec<ArgValue>,
}

#[derive(Debug, Default)]
struct Flags {
    for_command: bool,
    for_root: bool,
}

#[derive(Debug)]
enum ArgValue {
    NameWithType { name: Ident, ty: PathSegment },
    Type(PathSegment),
}

impl ArgValue {
    fn ty(&self) -> PathSegment {
        match self {
            Self::Type(ty) => ty.clone(),
            Self::NameWithType { ty, .. } => ty.to_owned(),
        }
    }

    fn is_serialize(&self) -> bool {
        let arg_type = self.ty().to_token_stream().to_string();
        arg_type == "Serialize"
    }
}

impl Parse for CreateCommand {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let docs = parse_docs(input)?;

        let mut variants = vec![];
        while !input.is_empty() {
            let (name, flags) = parse_name_and_flags(input)?;
            let term_type = parse_term_type(input, &name)?;
            let args = parse_args(input)?;
            variants.push(Variant {
                flags,
                name,
                term_type,
                args,
            });
        }

        let parsed = CreateCommand { docs, variants };

        Ok(parsed)
    }
}

impl CreateCommand {
    pub(super) fn build(self) -> TokenStream {
        let CreateCommand { docs, variants } = self;

        let gen: Vec<TokenStream> = variants
            .into_iter()
            .flat_map(|variant| build_variant(&docs, variant))
            .collect();

        quote! { #(#gen)* }
    }
}

fn parse_docs(input: ParseStream) -> syn::Result<Vec<TokenTree>> {
    let mut docs: Vec<TokenTree> = vec![];
    while input.peek(Token![#]) {
        docs.push(input.parse()?);
        docs.push(input.parse()?);
    }

    Ok(docs)
}

// Parse
//
// `command_name,` or `only_root, command_name,`
fn parse_name_and_flags(input: ParseStream) -> syn::Result<(Ident, Flags)> {
    let mut flags = Flags {
        for_root: true,
        for_command: true,
    };

    let ident = input.parse::<Ident>()?;
    if input.peek(Token![,]) {
        input.parse::<Token![,]>()?;
    }

    if ident == "only_root" {
        flags.for_command = false;
    } else if ident == "only_command" {
        flags.for_root = false;
    } else {
        return Ok((ident, flags));
    }

    let name = input.parse()?;
    if input.peek(Token![,]) {
        input.parse::<Token![,]>()?;
    }
    Ok((name, flags))
}

fn parse_term_type(input: ParseStream, name: &Ident) -> syn::Result<Ident> {
    if !input.peek(Token![:]) {
        return Ok(Ident::new(
            &name.to_string().to_case(Case::UpperCamel),
            name.span(),
        ));
    }
    input.parse::<Token![:]>()?;

    let ident = input.parse()?;
    if input.peek(Token![,]) {
        input.parse::<Token![,]>()?;
    }
    Ok(ident)
}

fn parse_args(input: ParseStream) -> syn::Result<Vec<ArgValue>> {
    if input.peek(Ident) {
        let arg = input.parse()?;

        if input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
        }

        return Ok(vec![ArgValue::Type(arg)]);
    }

    if !input.peek(Paren) {
        return Ok(vec![]);
    }

    let content;
    parenthesized!(content in input);

    let mut types: Vec<ArgValue> = vec![];
    while content.peek(Ident) {
        if content.peek2(Token![:]) {
            let name = content.parse()?;
            content.parse::<Token![:]>()?;
            let ty = content.parse()?;
            types.push(ArgValue::NameWithType { name, ty });
        } else {
            types.push(ArgValue::Type(content.parse()?));
        }

        if content.peek(Token![,]) {
            content.parse::<Token![,]>()?;
        }
    }

    if input.peek(Token![,]) {
        input.parse::<Token![,]>()?;
    }

    Ok(types)
}

fn gen_arg_name(i: usize, arg: &ArgValue) -> Ident {
    match arg {
        ArgValue::NameWithType { name, .. } => name.clone(),
        ArgValue::Type(ty) => {
            let arg_name_f = if i == 0 {
                "arg".into()
            } else {
                format!("arg{}", i + 1)
            };
            Ident::new(&arg_name_f, ty.ident.span())
        }
    }
}

fn build_variant(docs: &[TokenTree], variant: Variant) -> Vec<TokenStream> {
    let Variant {
        flags,
        name,
        term_type,
        args,
    } = variant;

    let mut args_decl: Vec<TokenStream> = args
        .iter()
        .enumerate()
        .map(|(i, arg)| {
            let arg_name = gen_arg_name(i, arg);
            let arg_type = arg.ty();
            quote! { #arg_name: impl #arg_type }
        })
        .collect();
    let mut all_args = vec![quote! { self }];
    all_args.append(&mut args_decl);

    let cmd_body: Vec<TokenStream> = args
        .iter()
        .enumerate()
        .map(|(i, arg)| {
            let arg_name = gen_arg_name(i, arg);
            if arg.is_serialize() {
                quote! { let cmd = cmd.with_arg(Command::from_json(#arg_name)); }
            } else {
                quote! { let cmd = #arg_name.with_cmd(cmd); }
            }
        })
        .collect();

    let mut gen = vec![];

    if flags.for_root {
        gen.push(quote! {
            impl crate::r {
                #[allow(clippy::should_implement_trait)]
                #[allow(clippy::too_many_arguments)]
                #(#docs)*
                pub fn #name(#(#all_args),*) -> Command {
                    let cmd = Command::new(TermType::#term_type);
                    #(#cmd_body)*
                    cmd
                }
            }
        });
    }

    if flags.for_command {
        gen.push(quote! {
            impl crate::Command {
                #[allow(clippy::should_implement_trait)]
                #[allow(clippy::too_many_arguments)]
                #(#docs)*
                pub fn #name(#(#all_args),*) -> Command {
                    let cmd = Command::new(TermType::#term_type);
                    #(#cmd_body)*
                    cmd.with_parent(self)
                }
            }
        });
    }

    gen
}
