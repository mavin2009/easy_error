// easy_error_macros/src/lib.rs

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, Error, Expr, Ident, LitStr, Token,
};

/// Macro to enhance the `?` operator by adding context.
#[proc_macro]
pub fn try_easy(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as TryEasyInput);
    let expr = input.expr;
    let context = input.context;

    // Generate code that adds context to the error using `map_err`.
    let expanded = quote! {
        #expr.map_err(|e| ::easy_error_core::EasyError::with_context(e, #context))?
    };

    TokenStream::from(expanded)
}

/// Struct to parse the input for `try_easy!` macro.
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
/// # Arguments
///
/// * `name` - The name of the error enum.
/// * `variants` - A list of variant names.
///
/// # Example
///
/// ```rust
/// use my_easy_error_macros::{define_error, try_easy};
/// use my_easy_error_core::EasyError;
///
/// define_error!(MyError, IoError, ParseError);
/// ```
#[proc_macro]
pub fn define_error(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DefineErrorInput);
    let name = input.name;
    let variants = input.variants;

    // Check if variants are provided
    if variants.is_empty() {
        // Emit a compile-time error if no variants are provided
        return Error::new(name.span(), "define_error! requires at least one variant")
            .to_compile_error()
            .into();
    }

    // Generate enum variants
    let variants_enum = quote! {
        #(#variants),*
    };

    // Generate a single `From` implementation that matches on all variants
    let from_impl = quote! {
        impl From<#name> for EasyError {
            fn from(error: #name) -> Self {
                match error {
                    #(
                        #name::#variants => EasyError::with_context(error, stringify!(#variants)),
                    )*
                }
            }
        }
    };

    // Generate the error type with implementations
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

    // Convert the expanded code into a TokenStream
    TokenStream::from(expanded)
}

/// Struct to parse the input for `define_error!` macro.
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
