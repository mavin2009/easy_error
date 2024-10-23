// easy_error_macros/tests/compile-fail/invalid_try_easy.rs

use easy_error_core::EasyError;
use easy_error_macros::try_easy;

fn some_operation() -> Result<(), &'static str> {
    Err("operation failed")
}

fn main() -> Result<(), ()> {
    // Incorrect context type (not a string literal)
    try_easy!(some_operation(), 123);
    Ok(())
}
