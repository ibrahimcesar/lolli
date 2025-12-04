//! # lolli-core
//!
//! Core data structures for the Lolli linear logic workbench.
//!
//! Linear logic is a resource-aware logic where hypotheses must be used exactly once.
//! This crate provides the fundamental types for working with linear logic formulas,
//! sequents, proofs, and extracted terms.
//!
//! ## Linear Logic Connectives
//!
//! | Connective | Symbol | Name | Meaning |
//! |------------|--------|------|---------|
//! | Tensor | A ⊗ B | "times" | Both A and B (independently) |
//! | Par | A ⅋ B | "par" | A or B (opponent chooses) |
//! | Lolli | A ⊸ B | "lollipop" | Consume A, produce B |
//! | With | A & B | "with" | Both available, choose one |
//! | Plus | A ⊕ B | "plus" | One of them (I choose) |
//! | Of course | !A | "bang" | Unlimited supply of A |
//! | Why not | ?A | "whynot" | Demand for A |

#![warn(missing_docs)]
#![warn(clippy::all)]

pub mod formula;
pub mod proof;
pub mod sequent;
pub mod term;

pub use formula::Formula;
pub use proof::{Proof, Rule};
pub use sequent::{Sequent, TwoSidedSequent};
pub use term::Term;
