# easy_error

`easy_error` is a Rust crate that provides macros and utilities to simplify and enhance error handling. It reduces boilerplate, improves readability, and maintains Rust's safety guarantees.

## FAQ

### Why use `easy_error` over existing error handling crates?

`easy_error` offers a streamlined approach to error handling by combining powerful macros with minimal boilerplate, enhancing readability and maintainability.

### Can I use `easy_error` with other error handling strategies?

Absolutely! `easy_error` is designed to integrate seamlessly with popular crates like `thiserror` and `anyhow`, providing flexibility to fit various project needs.

## Features

- **Enhanced `?` Operator:** Propagate errors with additional context effortlessly.
- **Custom Error Definitions:** Define custom error types with minimal boilerplate.
- **Integration with Popular Crates:** Seamlessly integrates with `thiserror` and `anyhow`.

## Usage

### Basic example

use easy_error::{define_error, try_easy, EasyError};

define_error!(MyError, IoError, ParseError);

fn read_file() -> Result<String, EasyError> {
    let content = try_easy!(std::fs::read_to_string("path/to/file.txt"), "Failed to read file");
    Ok(content)
}

fn main() -> Result<(), EasyError> {
    let file_content = read_file()?;
    println!("File Content: {}", file_content);
    Ok(())
}


### Using with thiserror

Integrate easy_error with thiserror for more detailed and customized error handling.

```rust
use easy_error::{define_error, try_easy, EasyError};
use thiserror::Error;

define_error!(MyError, IoError, ParseError);

#[derive(Error, Debug)]
pub enum AppError {
    #[error("IO Error: {0}")]
    Io(#[from] MyError),

    #[error("Parse Error: {0}")]
    Parse(#[from] MyError),

    #[error("Unknown error")]
    Unknown,
}

fn read_and_parse() -> Result<(), AppError> {
    let content = try_easy!(std::fs::read_to_string("path/to/file.txt"), "Failed to read file");
    // Simulate parsing
    if content.is_empty() {
        return Err(AppError::Parse(MyError::ParseError));
    }
    Ok(())
}

fn main() -> Result<(), AppError> {
    read_and_parse()?;
    println!("File read and parsed successfully.");
    Ok(())
}


### Using with anyhow

Leverage easy_error alongside anyhow for flexible error handling without the need for defining custom error types.

```rust
use easy_error::{try_easy, EasyError};
use anyhow::{Context, Result};

fn read_file() -> Result<String, EasyError> {
    let content = try_easy!(std::fs::read_to_string("path/to/file.txt"), "Failed to read file");
    Ok(content)
}

fn main() -> Result<()> {
    let content = read_file()
        .context("Unable to complete the read_file operation")?;
    println!("File Content: {}", content);
    Ok(())
}




### Defining Custom Errors

Use the `define_error!` macro to create custom error types.

```rust
use easy_error::{define_error, EasyError};

define_error!(MyError, IoError, ParseError);

fn perform_task() -> Result<(), EasyError> {
    // Your code here
    Ok(())
}


### Changelog

```markdown
# Changelog

## [0.2.0] - 2024-10-22
### Added
- Enhanced `define_error!` macro with input validation.
- Integration examples with `thiserror` and `anyhow`.

### Fixed
- Resolved cyclic dependency issues within the workspace.
- Improved error messages for compile-fail tests.

## [0.1.0] - 2024-09-15
- Initial release with basic `define_error!` and `try_easy!` macros.