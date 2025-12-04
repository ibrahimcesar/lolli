//! # lolli-parse
//!
//! Parser for the Lolli linear logic workbench.
//!
//! This crate provides parsing functionality for linear logic formulas and sequents.
//!
//! ## Example
//!
//! ```ignore
//! use lolli_parse::{parse_formula, parse_sequent};
//!
//! let formula = parse_formula("A -o B").unwrap();
//! let sequent = parse_sequent("A, B |- A * B").unwrap();
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]

// Parser implementation will be added in Issue #5 and #6
// For now, we just re-export core types

pub use lolli_core::{Formula, Sequent, TwoSidedSequent};

/// Parse error type (placeholder).
#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    /// Unexpected token in input
    #[error("Unexpected token: {0}")]
    UnexpectedToken(String),

    /// Unknown operator
    #[error("Unknown operator: {0}")]
    UnknownOperator(String),

    /// Unexpected rule during parsing
    #[error("Unexpected rule: {0}")]
    UnexpectedRule(String),

    /// General parse error
    #[error("Parse error: {0}")]
    General(String),
}

/// Parse a formula from a string (placeholder).
///
/// # Errors
///
/// Returns a `ParseError` if the input is not a valid formula.
pub fn parse_formula(_input: &str) -> Result<Formula, ParseError> {
    // TODO: Implement in Issue #6
    Err(ParseError::General("Parser not yet implemented".to_string()))
}

/// Parse a sequent from a string (placeholder).
///
/// # Errors
///
/// Returns a `ParseError` if the input is not a valid sequent.
pub fn parse_sequent(_input: &str) -> Result<TwoSidedSequent, ParseError> {
    // TODO: Implement in Issue #6
    Err(ParseError::General("Parser not yet implemented".to_string()))
}
