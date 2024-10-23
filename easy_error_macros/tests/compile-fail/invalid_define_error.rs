// easy_error_macros/tests/compile-fail/invalid_define_error.rs

use easy_error_macros::define_error;

// Missing variants
define_error!(IncompleteError);

fn main() {}
