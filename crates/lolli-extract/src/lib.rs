//! # lolli-extract
//!
//! Term extraction for the Lolli linear logic workbench.
//!
//! This crate implements the Curry-Howard correspondence, extracting
//! computational content (lambda terms) from linear logic proofs.

#![warn(missing_docs)]
#![warn(clippy::all)]

pub use lolli_core::{Proof, Term};

/// Term extractor (placeholder).
///
/// Extracts lambda terms from proofs via Curry-Howard.
pub struct Extractor {
    var_counter: usize,
}

impl Default for Extractor {
    fn default() -> Self {
        Self::new()
    }
}

impl Extractor {
    /// Create a new extractor.
    pub fn new() -> Self {
        Self { var_counter: 0 }
    }

    /// Generate a fresh variable name.
    pub fn fresh_var(&mut self) -> String {
        let v = format!("x{}", self.var_counter);
        self.var_counter += 1;
        v
    }

    /// Extract a term from a proof.
    ///
    /// The proof should be cut-free for best results.
    pub fn extract(&mut self, _proof: &Proof) -> Term {
        // TODO: Implement in Issue #13
        Term::Unit
    }
}
