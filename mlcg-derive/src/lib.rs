#![feature(try_blocks)]
use std::path::PathBuf;

use quote::{format_ident, quote};

#[proc_macro_derive(Eval)]
pub fn derive_eval_for_self(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(tokens as syn::DeriveInput);
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let expanded = quote::quote! {
        impl #impl_generics Eval<#name #ty_generics> for #name #ty_generics #where_clause {
            #[inline]
            fn eval(self) -> #name #ty_generics {
                self
            }
        }
    };
    expanded.into()
}

#[proc_macro]
pub fn commands(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    use std::fs;
    let expanded: Result<_, Box<dyn std::error::Error>> = try {
        let path = PathBuf::from(&std::env::var("CARGO_MANIFEST_DIR")?).join(
            tokens
                .to_string()
                .trim_start_matches("\"")
                .trim_end_matches("\""),
        );
        let json = serde_json::from_reader::<_, serde_json::Value>(
            fs::File::open(&path).map_err(|e| format!("{e} at {}", path.to_string_lossy()))?,
        )?;
        enum_template("command", json.as_object().expect("invalid template"), 0)
    };
    expanded.unwrap().into()
}

fn enum_template(
    command: &str,
    map: &serde_json::Map<String, serde_json::Value>,
    depth: usize,
) -> proc_macro2::TokenStream {
    let enum_name = format_ident!("{}", to_uppercase(command));

    let mut variants = quote::quote! {};
    let mut writes = quote::quote! {};
    let mut froms = quote::quote! {};
    let mut sub_commands = quote::quote! {};
    for (command, fields) in map {
        if command == "padding" {
            continue;
        }

        let command_uppercase = format_ident!("{}", to_uppercase(command));

        let command_ty = if depth == 0 {
            let command_mod_name = format_ident!("{}", command);
            quote::quote! { #command_mod_name::#command_uppercase }
        } else {
            quote::quote! { #command_uppercase}
        };

        variants.extend(quote::quote! { #command_uppercase(#command_ty), });
        writes.extend(quote::quote! { Self::#command_uppercase(sc) => sc.fmt(f), });
        froms.extend(quote! {
            impl From<#command_ty> for #enum_name {
                fn from(sc: #command_ty) -> Self {
                    Self::#command_uppercase(sc)
                }
            }
        });

        let padding = fields.get("padding").map(|padding| {
            padding
                .as_number()
                .expect("invalid template: padding is not unsigned number")
                .as_u64()
                .expect("invalid template: padding is not unsigned integer")
        });

        match fields {
            serde_json::Value::Array(fields) => {
                sub_commands.extend(struct_template(command, fields, padding, depth + 1))
            }
            serde_json::Value::Object(sub_command) => {
                sub_commands.extend(enum_template(command, sub_command, depth + 1))
            }
            serde_json::Value::Number(_padding) => {}
            _ => panic!("invalid template"),
        }
    }

    let define = quote::quote! {
        #[derive(Debug, Clone)]
        pub enum #enum_name {
            #variants
        }

        impl std::fmt::Display for #enum_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    #writes
                }
            }
        }

        #froms

        #sub_commands

    };
    wrap_in_module(depth, command, define)
}

fn struct_template(
    command: &str,
    fields: &[serde_json::Value],
    padding: Option<u64>,
    depth: usize,
) -> proc_macro2::TokenStream {
    let struct_name = format_ident!("{}", to_uppercase(command));

    let padding = padding
        .map(|padding| padding as usize - fields.len())
        .unwrap_or_default();

    let writes = fields
        .iter()
        .map(|field| {
            let field = format_ident!("{}", field.as_str().expect("invalid template"));
            quote::quote! { write!(f, " {}", self.#field)?; }
        })
        .chain((0..padding).map(|_| quote::quote! {write!(f, " 0")?;}));

    let fields = fields
        .iter()
        .map(|field| format_ident!("{}", field.as_str().expect("invalid template")));
    let struct_define = quote::quote! {
        #[derive(Debug, Clone)]
        pub struct #struct_name {
            #(pub #fields: crate::String,)*
        }

        impl std::fmt::Display for #struct_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, #command)?;
                #( #writes )*
                Ok(())
            }
        }
    };

    wrap_in_module(depth, command, struct_define)
}

fn wrap_in_module(
    depth: usize,
    command: &str,
    define: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    if depth == 1 {
        let mod_name = format_ident!("{}", command);
        quote::quote! {
            #[allow(non_snake_case)]
            pub mod #mod_name {
                #define
            }
        }
    } else {
        define
    }
}

fn to_uppercase(src: &str) -> String {
    let mut string = src.to_owned();
    unsafe { string.as_bytes_mut()[0] = string.as_bytes_mut()[0].to_ascii_uppercase() };
    string
}
