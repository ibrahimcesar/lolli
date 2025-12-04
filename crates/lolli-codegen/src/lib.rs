//! # lolli-codegen
//!
//! Code generation for the Lolli linear logic workbench.
//!
//! This crate generates Rust code from linear logic proofs,
//! translating formulas to types and terms to code.

#![warn(missing_docs)]
#![warn(clippy::all)]

pub use lolli_core::{Formula, Proof, Term};

/// Rust code generator (placeholder).
pub struct RustCodegen {
    indent: usize,
}

impl Default for RustCodegen {
    fn default() -> Self {
        Self::new()
    }
}

impl RustCodegen {
    /// Create a new code generator.
    pub fn new() -> Self {
        Self { indent: 0 }
    }

    /// Generate a Rust type from a linear logic formula.
    pub fn formula_to_type(&self, formula: &Formula) -> String {
        match formula {
            Formula::Atom(a) => a.clone(),
            Formula::NegAtom(a) => format!("{}Dual", a),
            Formula::Tensor(a, b) => {
                format!("({}, {})", self.formula_to_type(a), self.formula_to_type(b))
            }
            Formula::Par(a, b) => {
                format!("Par<{}, {}>", self.formula_to_type(a), self.formula_to_type(b))
            }
            Formula::Lolli(a, b) => {
                format!(
                    "impl FnOnce({}) -> {}",
                    self.formula_to_type(a),
                    self.formula_to_type(b)
                )
            }
            Formula::With(a, b) => {
                format!("With<{}, {}>", self.formula_to_type(a), self.formula_to_type(b))
            }
            Formula::Plus(a, b) => {
                format!(
                    "Either<{}, {}>",
                    self.formula_to_type(a),
                    self.formula_to_type(b)
                )
            }
            Formula::OfCourse(a) => format!("Rc<{}>", self.formula_to_type(a)),
            Formula::WhyNot(a) => format!("Demand<{}>", self.formula_to_type(a)),
            Formula::One => "()".to_string(),
            Formula::Bottom => "!".to_string(),
            Formula::Top => "Top".to_string(),
            Formula::Zero => "Void".to_string(),
        }
    }

    /// Generate Rust code from an extracted term.
    pub fn term_to_code(&mut self, term: &Term) -> String {
        match term {
            Term::Var(v) => v.clone(),
            Term::Unit => "()".to_string(),
            Term::Pair(a, b) => format!("({}, {})", self.term_to_code(a), self.term_to_code(b)),
            Term::Abs(x, body) => format!("|{}| {}", x, self.term_to_code(body)),
            Term::App(f, a) => format!("{}({})", self.term_to_code(f), self.term_to_code(a)),
            Term::Inl(a) => format!("Either::Left({})", self.term_to_code(a)),
            Term::Inr(b) => format!("Either::Right({})", self.term_to_code(b)),
            Term::Fst(p) => format!("{}.0", self.term_to_code(p)),
            Term::Snd(p) => format!("{}.1", self.term_to_code(p)),
            Term::Promote(a) => format!("Rc::new({})", self.term_to_code(a)),
            _ => "/* TODO */".to_string(),
        }
    }
}
