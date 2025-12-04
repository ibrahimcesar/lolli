//! # lolli-prove
//!
//! Proof search for the Lolli linear logic workbench.
//!
//! This crate implements focused proof search for linear logic,
//! supporting MALL (Multiplicative-Additive) and MELL (with exponentials).

#![warn(missing_docs)]
#![warn(clippy::all)]

pub use lolli_core::{Proof, Rule, Sequent};

/// Focused prover for linear logic (placeholder).
///
/// Uses Andreoli's focused sequent calculus for efficient proof search.
pub struct FocusedProver {
    /// Maximum search depth
    pub max_depth: usize,
    /// Enable caching of failed branches
    pub use_cache: bool,
}

impl FocusedProver {
    /// Create a new prover with the given maximum depth.
    pub fn new(max_depth: usize) -> Self {
        Self {
            max_depth,
            use_cache: true,
        }
    }

    /// Attempt to prove the given sequent.
    ///
    /// Returns `Some(proof)` if provable, `None` otherwise.
    pub fn prove(&mut self, _sequent: &Sequent) -> Option<Proof> {
        // TODO: Implement in Issues #8-#11
        None
    }
}
