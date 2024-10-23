// easy_error_workspace/easy_error_macros/src/lib.rs
use proc_macro::TokenStream;
use quote::quote;
use std::collections::HashSet;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, Error, Expr, Ident, LitStr, Token,
};

/// Macro to enhance the `?` operator by adding context.
#[proc_macro]
pub fn try_easy(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as TryEasyInput);
    let expr = input.expr;
    let context = input.context;

    let expanded = quote! {
        #expr.map_err(|e| ::easy_error_core::EasyError::with_context(e, #context))?
    };

    TokenStream::from(expanded)
}

struct TryEasyInput {
    expr: Expr,
    _comma: Token![,],
    context: LitStr,
}

impl Parse for TryEasyInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(TryEasyInput {
            expr: input.parse()?,
            _comma: input.parse()?,
            context: input.parse()?,
        })
    }
}

/// Defines a custom error enum with specified variants.
///
/// This macro generates a `Debug`, `Display`, and `Error` implementation for the enum,
/// and a `From<YourError>` implementation for `::easy_error_core::EasyError`.
///
/// # Arguments
///
/// * `name`: The name of the error enum (an identifier).
/// * `variants`: A comma-separated list of variant names (identifiers). At least one variant is required.
///
/// # Example
///
/// ```rust
/// use easy_error_macros::define_error;
///
/// define_error!(MyError, IoError, ParseError, NetworkError);
///
/// // ... usage of MyError
/// ```
///
/// # Errors
///
/// This macro will emit compile-time errors if:
///
/// * No variants are provided.
/// * Duplicate variant names are detected.
///
#[proc_macro]
pub fn define_error(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DefineErrorInput);
    let name = input.name;
    let variants = input.variants;

    // Check for duplicate variant names
    let mut variant_names = HashSet::new();
    let mut errors = Vec::new();
    for variant in &variants {
        if !variant_names.insert(variant.to_string()) {
            let error = Error::new(
                variant.span(),
                format!("Duplicate variant name: {}", variant),
            );
            errors.push(error);
        }
    }

    if !errors.is_empty() {
        let combined_error = errors.into_iter().reduce(|mut acc, err| {
            acc.combine(err);
            acc
        });
        return combined_error.unwrap().to_compile_error().into();
    }

    // Check if variants are provided after checking for duplicates
    if variants.is_empty() {
        return Error::new(name.span(), "define_error! requires at least one variant")
            .to_compile_error()
            .into();
    }

    let variants_enum = quote! {
        #(#variants),*
    };

    let from_impl = quote! {
        impl From<#name> for ::easy_error_core::EasyError {
            fn from(error: #name) -> Self {
                match error {
                    #(
                        #name::#variants => ::easy_error_core::EasyError::with_context(error, stringify!(#variants)),
                    )*
                }
            }
        }
    };

    let expanded = quote! {
        #[derive(Debug)]
        pub enum #name {
            #variants_enum
        }

        impl std::fmt::Display for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    #(
                        #name::#variants => write!(f, stringify!(#variants)),
                    )*
                }
            }
        }

        impl std::error::Error for #name {}

        #from_impl
    };

    TokenStream::from(expanded)
}

struct DefineErrorInput {
    name: Ident,
    variants: Vec<Ident>,
}

impl Parse for DefineErrorInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name = input.parse()?;
        let mut variants = Vec::new();

        if input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
            while !input.is_empty() {
                let variant = input.parse()?;
                variants.push(variant);
                if input.peek(Token![,]) {
                    input.parse::<Token![,]>()?;
                } else {
                    break;
                }
            }
        }

        Ok(DefineErrorInput { name, variants })
    }
}
