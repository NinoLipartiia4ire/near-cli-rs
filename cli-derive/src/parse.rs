use proc_macro2::TokenStream;
use syn::{Ident, Type};
use quote::quote;

use crate::types::{StructArgs, FieldArgs};


pub fn gen_interactive(args: &StructArgs) -> TokenStream {
    let struct_ident = &args.ident;
    let fields = gen_interactive_fields(args);

    quote! {
        impl near_cli_visual::types::Interactive for #struct_ident {
            fn interactive(clap: Option<&Self::Clap>, mut builder: Self::Builder) -> Self::Builder {
                if let Some(clap) = clap {
                    #(#fields)*
                }

                builder
            }
        }
    }
}

fn gen_interactive_fields(args: &StructArgs) -> Vec<TokenStream> {
    let struct_ident = &args.ident;
    args.fields().into_iter().map(|field| {
        let FieldArgs {
            ident: field_ident,
            ty,
            prompt_msg,
            prompt_fn,
            ..
        } = field;

        if prompt_msg.is_none() && prompt_fn.is_none() {
            // Skip if not present
            return quote!();
        }

        let field_ident = field_ident.as_ref().expect("Enum/tuples/newtypes are unsupported");
        let mut prompter = None;
        if let Some(prompt_msg) = prompt_msg {
            prompter = Some(quote! { near_cli_visual::prompt_input_with_msg(#prompt_msg) });
        }
        else if let Some(prompt_fn) = prompt_fn {
            let prompt_fn = syn::Ident::new(&prompt_fn, struct_ident.span());
            prompter = Some(quote! { #prompt_fn () });
        }
        let interactive = prompter.expect(
            &format!("Did not specify how to prompt {}::{} with either prompt_msg or prompt_fn",
                struct_ident, field_ident));

        let builder_fn = syn::Ident::new(&format!("set_{}", field_ident), struct_ident.span());

        // quote! {
        //     let value = clap . #field_ident . as_ref().unwrap_or_else(|| {
        //         #interactive
        //     });
        //     let builder = builder . #builder_fn (value)

        // }

        quote! {
            builder = builder . #builder_fn (
                match clap . #field_ident . as_ref() {
                    Some(value) => value.clone(),
                    None => #interactive,
                }
            );
        }
    })
    .collect()
}

pub fn gen_build(args: &StructArgs) -> TokenStream {
    let struct_ident = &args.ident;
    let build_retry_loop = gen_build_retry_loop();
    let subcommand = gen_build_subcommand(args);
    let fields = gen_build_fields(args);

    quote! {
        impl near_cli_visual::types::Build for #struct_ident {
            type Err = ();

            fn build(clap: Option<Self::Clap>, mut builder: Self::Builder) -> Result<Self, Self::Err> {
                let scope = #build_retry_loop;

                Ok(Self {
                    #(#fields)*

                    #subcommand
                })
            }
        }

    }
}

// The loop where we call into Interactive/Validate
pub fn gen_build_retry_loop() -> TokenStream {
    quote! {{
        let mut count = near_cli_visual::consts::max_build_retry();
        let scope = loop {
            builder = <Self as near_cli_visual::types::Interactive>::interactive(clap.as_ref(), builder);
            let valid = <Self as near_cli_visual::types::Validate>::validate(clap.as_ref(), &builder);

            count -= 1;
            if valid.is_ok() {
                break Ok(builder);
            }
            else if count == 0 {
                // break Err(Self::Err::None);
                // break Err(valid.unwrap_err());
                break Err(());
            }
        }?
        .into_scope()
    }}
}

pub fn gen_build_subcommand(args: &StructArgs) -> TokenStream {
    let struct_ident = &args.ident;
    if let Some((sub_ident, single, sub_ty, prompt_msg)) = subcommand_details(args) {
        let enum_sub_interactive = if single {
            quote! {{ clap.unwrap_single_subcommand() }}
        } else {
            // let prompt_msg = prompt_msg.expect("prompt_msg required for choosing subcommand");
            // TODO: try to get enum version working too. the above doesn't do much. Need inner value, Probably need a #[derive(Eclap)] for enum
            // quote! {{ near_cli_visual::prompt_variant :: <  > () }}
            quote!()
        };

        return quote! {
            #sub_ident : {
                // Here, we're trying to get inner value of the enum
                let mut sub_clap = None;
                if let Some(clap) = clap {
                    if let Some(clap) = clap.subcommand {
                        sub_clap = Some(#enum_sub_interactive)
                    }
                }

                // let subcommand = #sub_ty :: build :: <Self> (sub_clap, scope)?;

                let sub_builder = <#sub_ty as types::BuilderFrom<Self>>::builder_from(&scope);
                let subcommand = <#sub_ty as types::Build>::build(sub_clap, sub_builder)?;

                subcommand
            },
        };
    }

    quote!()
}

fn gen_build_fields(args: &StructArgs) -> Vec<TokenStream> {
    args.fields().into_iter().map(|field| {
        let FieldArgs {
            ident: field_ident,
            ty,
            subcommand,
            ..
        } = field;

        if *subcommand {
            return quote!();
        }

        let field_ident = field_ident.as_ref().expect("Enum/tuples/newtypes are unsupported");
        // let builder_fn = syn::Ident::new(&format!("set_{}", field_ident), struct_ident.span());

        quote! {
            #field_ident : scope . #field_ident,
        }
    })
    .collect()
}

fn subcommand_details(args: &StructArgs) -> Option<(Ident, bool, Type, TokenStream)> {
    for FieldArgs { ident, ty, single, subcommand, prompt_msg, prompt_fn, ..} in args.fields() {
        if *subcommand {
            let ident = ident.as_ref().expect("Enum/tuple/newtypes not supported").clone();
            return Some((ident, *single, ty.clone(), quote!()));
        }
    }

    None
}