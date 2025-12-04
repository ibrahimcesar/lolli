//! Linear logic formula representation.
//!
//! This module provides the [`Formula`] enum representing linear logic formulas
//! with all standard connectives.

/// A linear logic formula.
///
/// Linear logic has a rich set of connectives split into multiplicative and additive families,
/// plus exponentials for controlled structural rules.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Formula {
    // Atoms
    /// Atomic proposition
    Atom(String),
    /// Negated atomic proposition (A⊥)
    NegAtom(String),

    // Multiplicatives
    /// Tensor product (A ⊗ B) - "both A and B independently"
    Tensor(Box<Formula>, Box<Formula>),
    /// Par (A ⅋ B) - "A or B, opponent chooses"
    Par(Box<Formula>, Box<Formula>),
    /// Multiplicative unit (1)
    One,
    /// Multiplicative false (⊥)
    Bottom,

    // Additives
    /// With (A & B) - "both available, you choose one"
    With(Box<Formula>, Box<Formula>),
    /// Plus (A ⊕ B) - "one of them, I choose which"
    Plus(Box<Formula>, Box<Formula>),
    /// Additive truth (⊤)
    Top,
    /// Additive false (0)
    Zero,

    // Exponentials
    /// Of course (!A) - "unlimited supply of A, can copy/discard"
    OfCourse(Box<Formula>),
    /// Why not (?A) - "demand for A"
    WhyNot(Box<Formula>),

    // Derived (syntactic sugar)
    /// Linear implication (A ⊸ B) - sugar for A⊥ ⅋ B
    Lolli(Box<Formula>, Box<Formula>),
}

impl Formula {
    /// Compute the linear negation of a formula.
    ///
    /// Linear negation is involutive: (A⊥)⊥ = A
    ///
    /// De Morgan dualities:
    /// - (A ⊗ B)⊥ = A⊥ ⅋ B⊥
    /// - (A ⅋ B)⊥ = A⊥ ⊗ B⊥
    /// - (A & B)⊥ = A⊥ ⊕ B⊥
    /// - (A ⊕ B)⊥ = A⊥ & B⊥
    /// - 1⊥ = ⊥
    /// - ⊥⊥ = 1
    /// - (!A)⊥ = ?(A⊥)
    /// - (?A)⊥ = !(A⊥)
    pub fn negate(&self) -> Formula {
        match self {
            Formula::Atom(a) => Formula::NegAtom(a.clone()),
            Formula::NegAtom(a) => Formula::Atom(a.clone()),

            Formula::Tensor(a, b) => {
                Formula::Par(Box::new(a.negate()), Box::new(b.negate()))
            }
            Formula::Par(a, b) => {
                Formula::Tensor(Box::new(a.negate()), Box::new(b.negate()))
            }
            Formula::One => Formula::Bottom,
            Formula::Bottom => Formula::One,

            Formula::With(a, b) => {
                Formula::Plus(Box::new(a.negate()), Box::new(b.negate()))
            }
            Formula::Plus(a, b) => {
                Formula::With(Box::new(a.negate()), Box::new(b.negate()))
            }
            Formula::Top => Formula::Zero,
            Formula::Zero => Formula::Top,

            Formula::OfCourse(a) => Formula::WhyNot(Box::new(a.negate())),
            Formula::WhyNot(a) => Formula::OfCourse(Box::new(a.negate())),

            Formula::Lolli(a, b) => {
                // (A ⊸ B)⊥ = (A⊥ ⅋ B)⊥ = A ⊗ B⊥
                Formula::Tensor(a.clone(), Box::new(b.negate()))
            }
        }
    }

    /// Desugar the formula by expanding A ⊸ B to A⊥ ⅋ B.
    pub fn desugar(&self) -> Formula {
        match self {
            Formula::Lolli(a, b) => {
                Formula::Par(Box::new(a.negate().desugar()), Box::new(b.desugar()))
            }
            Formula::Tensor(a, b) => {
                Formula::Tensor(Box::new(a.desugar()), Box::new(b.desugar()))
            }
            Formula::Par(a, b) => {
                Formula::Par(Box::new(a.desugar()), Box::new(b.desugar()))
            }
            Formula::With(a, b) => {
                Formula::With(Box::new(a.desugar()), Box::new(b.desugar()))
            }
            Formula::Plus(a, b) => {
                Formula::Plus(Box::new(a.desugar()), Box::new(b.desugar()))
            }
            Formula::OfCourse(a) => Formula::OfCourse(Box::new(a.desugar())),
            Formula::WhyNot(a) => Formula::WhyNot(Box::new(a.desugar())),
            _ => self.clone(),
        }
    }

    /// Returns true if this formula is positive (async/eager).
    ///
    /// Positive formulas: ⊗, 1, ⊕, 0, !, atoms
    pub fn is_positive(&self) -> bool {
        matches!(
            self,
            Formula::Atom(_)
                | Formula::Tensor(_, _)
                | Formula::One
                | Formula::Plus(_, _)
                | Formula::Zero
                | Formula::OfCourse(_)
        )
    }

    /// Returns true if this formula is negative (sync/lazy).
    ///
    /// Negative formulas: ⅋, ⊥, &, ⊤, ?, negated atoms
    pub fn is_negative(&self) -> bool {
        !self.is_positive()
    }

    /// Pretty print the formula with Unicode symbols.
    pub fn pretty(&self) -> String {
        match self {
            Formula::Atom(a) => a.clone(),
            Formula::NegAtom(a) => format!("{}⊥", a),
            Formula::Tensor(a, b) => format!("({} ⊗ {})", a.pretty(), b.pretty()),
            Formula::Par(a, b) => format!("({} ⅋ {})", a.pretty(), b.pretty()),
            Formula::Lolli(a, b) => format!("({} ⊸ {})", a.pretty(), b.pretty()),
            Formula::With(a, b) => format!("({} & {})", a.pretty(), b.pretty()),
            Formula::Plus(a, b) => format!("({} ⊕ {})", a.pretty(), b.pretty()),
            Formula::OfCourse(a) => format!("!{}", a.pretty()),
            Formula::WhyNot(a) => format!("?{}", a.pretty()),
            Formula::One => "1".to_string(),
            Formula::Bottom => "⊥".to_string(),
            Formula::Top => "⊤".to_string(),
            Formula::Zero => "0".to_string(),
        }
    }

    /// Pretty print the formula with ASCII symbols.
    pub fn pretty_ascii(&self) -> String {
        match self {
            Formula::Atom(a) => a.clone(),
            Formula::NegAtom(a) => format!("{}^", a),
            Formula::Tensor(a, b) => format!("({} * {})", a.pretty_ascii(), b.pretty_ascii()),
            Formula::Par(a, b) => format!("({} | {})", a.pretty_ascii(), b.pretty_ascii()),
            Formula::Lolli(a, b) => format!("({} -o {})", a.pretty_ascii(), b.pretty_ascii()),
            Formula::With(a, b) => format!("({} & {})", a.pretty_ascii(), b.pretty_ascii()),
            Formula::Plus(a, b) => format!("({} + {})", a.pretty_ascii(), b.pretty_ascii()),
            Formula::OfCourse(a) => format!("!{}", a.pretty_ascii()),
            Formula::WhyNot(a) => format!("?{}", a.pretty_ascii()),
            Formula::One => "1".to_string(),
            Formula::Bottom => "bot".to_string(),
            Formula::Top => "top".to_string(),
            Formula::Zero => "0".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_negation_involutive() {
        let a = Formula::Atom("A".to_string());
        assert_eq!(a.negate().negate(), a);

        let complex = Formula::Tensor(
            Box::new(Formula::Atom("A".to_string())),
            Box::new(Formula::Atom("B".to_string())),
        );
        assert_eq!(complex.negate().negate(), complex);
    }

    #[test]
    fn test_de_morgan() {
        let a = Formula::Atom("A".to_string());
        let b = Formula::Atom("B".to_string());

        // (A ⊗ B)⊥ = A⊥ ⅋ B⊥
        let tensor = Formula::Tensor(Box::new(a.clone()), Box::new(b.clone()));
        let expected = Formula::Par(
            Box::new(Formula::NegAtom("A".to_string())),
            Box::new(Formula::NegAtom("B".to_string())),
        );
        assert_eq!(tensor.negate(), expected);
    }

    #[test]
    fn test_polarity() {
        assert!(Formula::Atom("A".to_string()).is_positive());
        assert!(Formula::NegAtom("A".to_string()).is_negative());
        assert!(Formula::Tensor(
            Box::new(Formula::Atom("A".to_string())),
            Box::new(Formula::Atom("B".to_string()))
        )
        .is_positive());
        assert!(Formula::Par(
            Box::new(Formula::Atom("A".to_string())),
            Box::new(Formula::Atom("B".to_string()))
        )
        .is_negative());
    }
}
