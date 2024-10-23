// easy_error_workspace/easy_error/examples/usage_example.rs

use easy_error::{define_error, try_easy, EasyError};
use std::env;
use std::fs::File;
use std::io::Read;

// Define custom error types with variants for different error scenarios.
define_error!(MyError, IoError, ParseError, ComputationError);

// Function to read the content of a file.
fn read_file(path: &str) -> Result<String, EasyError> {
    // Attempt to open the file. If it fails, wrap the error with context.
    let mut file = try_easy!(File::open(path), "Failed to open the specified file");

    let mut contents = String::new();

    // Attempt to read the file's contents. If it fails, wrap the error with context.
    try_easy!(
        file.read_to_string(&mut contents),
        "Failed to read the file's contents"
    );

    Ok(contents)
}

// Function to parse a number from the file's content.
fn parse_number(content: &str) -> Result<i32, EasyError> {
    // Attempt to parse the string into an i32. If it fails, wrap the error with context.
    let number = try_easy!(
        content.trim().parse::<i32>(),
        "Failed to parse a number from the file's content"
    );
    Ok(number)
}

// Function to compute the square of a number.
fn compute_square(number: i32) -> Result<i32, EasyError> {
    // Simulate a computation that could fail (e.g., overflow).
    number.checked_mul(number).ok_or_else(|| {
        // Manually create a custom error variant with context.
        EasyError::with_context(
            MyError::ComputationError,
            "Failed to compute the square of the number",
        )
    })
}

// Main function that orchestrates reading, parsing, and computing.
fn perform_task(path: &str) -> Result<(), EasyError> {
    // Step 1: Read the file.
    let content = read_file(path)?;
    println!("File Content: {}", content);

    // Step 2: Parse the number from the content.
    let number = parse_number(&content)?;
    println!("Parsed Number: {}", number);

    // Step 3: Compute the square of the number.
    let square = compute_square(number)?;
    println!("Square of the Number: {}", square);

    Ok(())
}

fn main() -> Result<(), EasyError> {
    // Collect command-line arguments.
    let args: Vec<String> = env::args().collect();

    // Check if the correct number of arguments is provided.
    if args.len() != 2 {
        println!("Usage: cargo run --example usage_example -- <file_path>");
        return Ok(());
    }

    let file_path = &args[1];

    perform_task(file_path)
}
