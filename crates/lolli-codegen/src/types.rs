//! Type generation from linear logic formulas.
//!
//! This module translates linear logic formulas into Rust types.

use lolli_core::Formula;

/// Generates Rust types from linear logic formulas.
pub struct TypeGenerator {
    /// Use explicit lifetime annotations
    pub use_lifetimes: bool,
}

impl Default for TypeGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl TypeGenerator {
    /// Create a new type generator.
    pub fn new() -> Self {
        Self {
            use_lifetimes: false,
        }
    }

    /// Generate a Rust type from a formula.
    pub fn generate(&self, formula: &Formula) -> String {
        match formula {
            // Atoms become type parameters or concrete types
            Formula::Atom(name) => name.clone(),
            Formula::NegAtom(name) => format!("{}Dual", name),

            // Tensor is a tuple - both values consumed together
            Formula::Tensor(a, b) => {
                format!("({}, {})", self.generate(a), self.generate(b))
            }

            // Par is the dual of tensor - continuation-style
            Formula::Par(a, b) => {
                format!("Par<{}, {}>", self.generate(a), self.generate(b))
            }

            // Linear implication is FnOnce - exactly one use
            Formula::Lolli(a, b) => {
                format!("impl FnOnce({}) -> {}", self.generate(a), self.generate(b))
            }

            // With is a lazy pair - can project to either
            Formula::With(a, b) => {
                format!("With<{}, {}>", self.generate(a), self.generate(b))
            }

            // Plus is a sum type - Either
            Formula::Plus(a, b) => {
                format!("Either<{}, {}>", self.generate(a), self.generate(b))
            }

            // Bang is Rc - can be cloned/dropped
            Formula::OfCourse(a) => {
                format!("Rc<{}>", self.generate(a))
            }

            // Why-not is demand for a bang
            Formula::WhyNot(a) => {
                format!("Demand<{}>", self.generate(a))
            }

            // Multiplicative units
            Formula::One => "()".to_string(),
            Formula::Bottom => "!".to_string(), // Never type

            // Additive units
            Formula::Top => "Top".to_string(), // Unit for &
            Formula::Zero => "Void".to_string(), // Empty type
        }
    }

    /// Generate a type with explicit ownership annotation.
    pub fn generate_owned(&self, formula: &Formula) -> String {
        let ty = self.generate(formula);
        // Linear types are always owned (moved)
        ty
    }

    /// Generate a type signature for a function argument.
    pub fn generate_arg(&self, formula: &Formula, name: &str) -> String {
        let ty = self.generate(formula);
        format!("{}: {}", name, ty)
    }

    /// Generate a return type.
    pub fn generate_return(&self, formulas: &[Formula]) -> String {
        match formulas.len() {
            0 => "()".to_string(),
            1 => self.generate(&formulas[0]),
            _ => {
                let types: Vec<String> = formulas.iter().map(|f| self.generate(f)).collect();
                format!("({})", types.join(", "))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_atoms() {
        let gen = TypeGenerator::new();
        assert_eq!(gen.generate(&Formula::atom("A")), "A");
        assert_eq!(gen.generate(&Formula::neg_atom("A")), "ADual");
    }

    #[test]
    fn test_tensor() {
        let gen = TypeGenerator::new();
        let tensor = Formula::tensor(Formula::atom("A"), Formula::atom("B"));
        assert_eq!(gen.generate(&tensor), "(A, B)");
    }

    #[test]
    fn test_lolli() {
        let gen = TypeGenerator::new();
        let lolli = Formula::lolli(Formula::atom("A"), Formula::atom("B"));
        assert_eq!(gen.generate(&lolli), "impl FnOnce(A) -> B");
    }

    #[test]
    fn test_plus() {
        let gen = TypeGenerator::new();
        let plus = Formula::plus(Formula::atom("A"), Formula::atom("B"));
        assert_eq!(gen.generate(&plus), "Either<A, B>");
    }

    #[test]
    fn test_with() {
        let gen = TypeGenerator::new();
        let with = Formula::with(Formula::atom("A"), Formula::atom("B"));
        assert_eq!(gen.generate(&with), "With<A, B>");
    }

    #[test]
    fn test_exponentials() {
        let gen = TypeGenerator::new();
        let bang = Formula::of_course(Formula::atom("A"));
        assert_eq!(gen.generate(&bang), "Rc<A>");

        let whynot = Formula::why_not(Formula::atom("A"));
        assert_eq!(gen.generate(&whynot), "Demand<A>");
    }

    #[test]
    fn test_units() {
        let gen = TypeGenerator::new();
        assert_eq!(gen.generate(&Formula::One), "()");
        assert_eq!(gen.generate(&Formula::Bottom), "!");
        assert_eq!(gen.generate(&Formula::Top), "Top");
        assert_eq!(gen.generate(&Formula::Zero), "Void");
    }

    #[test]
    fn test_complex() {
        let gen = TypeGenerator::new();
        // !A ⊗ B ⊸ C
        let formula = Formula::lolli(
            Formula::tensor(
                Formula::of_course(Formula::atom("A")),
                Formula::atom("B"),
            ),
            Formula::atom("C"),
        );
        assert_eq!(gen.generate(&formula), "impl FnOnce((Rc<A>, B)) -> C");
    }

    #[test]
    fn test_return_type() {
        let gen = TypeGenerator::new();

        assert_eq!(gen.generate_return(&[]), "()");
        assert_eq!(gen.generate_return(&[Formula::atom("A")]), "A");
        assert_eq!(
            gen.generate_return(&[Formula::atom("A"), Formula::atom("B")]),
            "(A, B)"
        );
    }
}
