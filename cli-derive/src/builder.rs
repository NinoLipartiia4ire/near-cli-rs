use crate::types::{StructArgs, FieldArgs};
use proc_macro2::TokenStream;
use quote::{quote};


pub fn gen(args: &StructArgs) -> TokenStream {
    let struct_ident = &args.ident;
    let builder_ident = format!("{}Builder", struct_ident);
    let builder_ident = syn::Ident::new(&builder_ident, struct_ident.span());
    let (funcs, fields) = gen_builder_internals(args);

    quote! {
        #[derive(Default)]
        struct #builder_ident {
            #(#fields)*
        }

        impl #builder_ident {
            #(#funcs)*
        }

        impl near_cli_visual::types::Builder for #struct_ident {
            type Builder = #builder_ident;
        }
    }
}

fn gen_builder_internals(args: &StructArgs) -> (Vec<TokenStream>, Vec<TokenStream>) {
    let StructArgs {
        ident: struct_ident,
        generics: _,
        data: _,
    } = args;

    args.fields().into_iter().map(|f| {
        let FieldArgs {
            ident,
            ty,
            subcommand,
            ..
        } = f;

        if *subcommand {
            // Subcommand are not apart of the Builder. So exclude it with empty field.
            return (quote! {}, quote! {});
        }

        // will fail if enum, newtype or tuple
        let ident = ident.as_ref().expect("only supported for regular structs");

        // Builder functions. This allows us to write `set_#field` into the builder.
        let builder_fn = syn::Ident::new(&format!("set_{}", ident), struct_ident.span());
        let builder_fn = quote! {
            fn #builder_fn (self, val: #ty) -> Self {
                self.#ident = Some(val);
                self
            }
        };

        let builder_field = quote! {
            #ident: Option<#ty>,
        };

        (builder_fn, builder_field)
    })
    .unzip()
}