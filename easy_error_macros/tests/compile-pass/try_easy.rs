// easy_error_macros/tests/compile-pass/try_easy.rs

use easy_error_core::EasyError;
use easy_error_macros::try_easy;

fn some_operation() -> Result<(), &'static str> {
    Ok(())
}

fn main() -> Result<(), EasyError> {
    try_easy!(some_operation(), "Failed to perform some_operation");
    Ok(())
}
