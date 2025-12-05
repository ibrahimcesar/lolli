//! # lolli-codegen
//!
//! Code generation for the Lolli linear logic workbench.
//!
//! This crate generates Rust code from linear logic proofs,
//! translating formulas to types and terms to code.
//!
//! ## Linear Logic to Rust Mapping
//!
//! | Linear Logic | Rust Type |
//! |--------------|-----------|
//! | A ⊸ B | `impl FnOnce(A) -> B` |
//! | A ⊗ B | `(A, B)` |
//! | A & B | `With<A, B>` (lazy pair) |
//! | A ⊕ B | `Either<A, B>` |
//! | 1 | `()` |
//! | ⊤ | `Top` (unit type) |
//! | 0 | `Void` (empty type) |
//! | !A | `Rc<A>` (shared) |
//!
//! ## Example
//!
//! ```
//! use lolli_codegen::RustCodegen;
//! use lolli_core::{Formula, Term};
//!
//! let codegen = RustCodegen::new();
//!
//! // Formula to type
//! let lolli = Formula::lolli(Formula::atom("A"), Formula::atom("B"));
//! assert_eq!(codegen.formula_to_type(&lolli), "impl FnOnce(A) -> B");
//!
//! // Term to code
//! let mut codegen = RustCodegen::new();
//! let id = Term::Abs("x".to_string(), Box::new(Term::Var("x".to_string())));
//! assert_eq!(codegen.term_to_code(&id), "|x| x");
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]

pub use lolli_core::{Formula, Proof, Sequent, Term};

mod types;
mod codegen;
mod prelude;

pub use types::TypeGenerator;
pub use codegen::RustCodegen;
pub use prelude::PRELUDE;

/// Generate a complete Rust function from a sequent and term.
///
/// # Example
///
/// ```
/// use lolli_codegen::{generate_function, Term};
/// use lolli_core::{Formula, Sequent};
///
/// let seq = lolli_core::TwoSidedSequent::new(
///     vec![Formula::atom("A"), Formula::atom("B")],
///     vec![Formula::tensor(Formula::atom("A"), Formula::atom("B"))],
/// );
/// let term = Term::Pair(
///     Box::new(Term::Var("a".to_string())),
///     Box::new(Term::Var("b".to_string())),
/// );
///
/// let code = generate_function("pair", &seq, &term);
/// assert!(code.contains("fn pair"));
/// ```
pub fn generate_function(name: &str, sequent: &lolli_core::TwoSidedSequent, term: &Term) -> String {
    let mut codegen = RustCodegen::new();
    codegen.generate_function(name, sequent, term)
}
