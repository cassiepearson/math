//! Errors enum

/// Errors enumeration
///
/// Lists all possible errors that can occur during the execution of the program
enum LibraryError {
    InvalidOperation(String),
    InvalidNumber(String),
    Parser(String),
    Other(String),
}

// TODO: implements error types for each module
// TODO: thiserror and anyhow integration
// TODO: Checked operations throughout the code
