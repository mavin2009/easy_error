// easy_error_macros/tests/compile-pass/define_error.rs

use easy_error_core::EasyError;
use easy_error_macros::define_error;
define_error!(MyError, IoError, ParseError);

fn main() {
    let error = MyError::IoError;
    println!("Defined error: {}", error);
}
