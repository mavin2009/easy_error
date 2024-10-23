// easy_error_workspace/easy_error/src/lib.rs

//! # easy_error
//!
//! `easy_error` is a Rust crate that provides macros and utilities to simplify and enhance error handling.
//! It reduces boilerplate, improves readability, and maintains Rust's safety guarantees.

pub use easy_error_core::EasyError;
pub use easy_error_macros::{define_error, try_easy};
