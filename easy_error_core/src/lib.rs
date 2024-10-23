// easy_error_workspace/easy_error_core/src/lib.rs

//! # easy_error_core
//!
//! `easy_error_core` provides the core `EasyError` type used for enhanced error handling.

#[derive(Debug)]
pub struct EasyError {
    source: Box<dyn std::error::Error + Send + Sync>,
    context: String,
}

impl EasyError {
    /// Creates a new `EasyError` with the given source error and context.
    pub fn with_context<E>(source: E, context: &str) -> Self
    where
        E: Into<Box<dyn std::error::Error + Send + Sync>>,
    {
        EasyError {
            source: source.into(),
            context: context.to_string(),
        }
    }

    /// Returns a reference to the source error.
    pub fn source(&self) -> &(dyn std::error::Error + 'static) {
        self.source.as_ref()
    }

    /// Returns the context string.
    pub fn context(&self) -> &str {
        &self.context
    }
}

impl std::fmt::Display for EasyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.context, self.source)
    }
}

impl std::error::Error for EasyError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&*self.source)
    }
}
