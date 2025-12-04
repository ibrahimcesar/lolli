//! Proof tree representation.
//!
//! This module provides data structures for representing proofs in sequent calculus.

use crate::{Formula, Sequent};

/// A proof in the sequent calculus.
#[derive(Clone, Debug)]
pub struct Proof {
    /// The conclusion of this proof step
    pub conclusion: Sequent,
    /// The inference rule applied
    pub rule: Rule,
    /// The premises (sub-proofs)
    pub premises: Vec<Proof>,
}

/// Inference rules for linear logic sequent calculus.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Rule {
    // Identity rules
    /// Axiom: ⊢ A⊥, A
    Axiom,
    /// Cut: from ⊢ Γ, A and ⊢ Δ, A⊥ derive ⊢ Γ, Δ
    Cut(Formula),

    // Multiplicative rules
    /// One introduction: ⊢ 1
    OneIntro,
    /// Bottom introduction: from ⊢ Γ derive ⊢ Γ, ⊥
    BottomIntro,
    /// Tensor introduction: from ⊢ Γ, A and ⊢ Δ, B derive ⊢ Γ, Δ, A ⊗ B
    TensorIntro,
    /// Par introduction: from ⊢ Γ, A, B derive ⊢ Γ, A ⅋ B
    ParIntro,

    // Additive rules
    /// Top introduction: ⊢ Γ, ⊤ (always provable)
    TopIntro,
    /// With introduction: from ⊢ Γ, A and ⊢ Γ, B derive ⊢ Γ, A & B
    WithIntro,
    /// Plus introduction (left): from ⊢ Γ, A derive ⊢ Γ, A ⊕ B
    PlusIntroLeft,
    /// Plus introduction (right): from ⊢ Γ, B derive ⊢ Γ, A ⊕ B
    PlusIntroRight,

    // Exponential rules
    /// Of course introduction: from ⊢ ?Γ, A derive ⊢ ?Γ, !A
    OfCourseIntro,
    /// Why not introduction: from ⊢ Γ, A derive ⊢ Γ, ?A
    WhyNotIntro,
    /// Weakening: from ⊢ Γ derive ⊢ Γ, ?A
    Weakening,
    /// Contraction: from ⊢ Γ, ?A, ?A derive ⊢ Γ, ?A
    Contraction,
    /// Dereliction: use of ?A as A
    Dereliction,

    // Focused proof search rules
    /// Focus on a positive formula at index
    FocusPositive(usize),
    /// Focus on a negative formula at index
    FocusNegative(usize),
    /// Blur (unfocus)
    Blur,
}

impl Proof {
    /// Count the number of cut rules in this proof.
    pub fn cut_count(&self) -> usize {
        let self_cuts = if matches!(self.rule, Rule::Cut(_)) {
            1
        } else {
            0
        };
        let premise_cuts: usize = self.premises.iter().map(|p| p.cut_count()).sum();
        self_cuts + premise_cuts
    }

    /// Returns true if this proof is cut-free.
    pub fn is_cut_free(&self) -> bool {
        self.cut_count() == 0
    }

    /// Returns the depth of the proof tree.
    pub fn depth(&self) -> usize {
        if self.premises.is_empty() {
            1
        } else {
            1 + self.premises.iter().map(|p| p.depth()).max().unwrap_or(0)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cut_count() {
        let axiom = Proof {
            conclusion: Sequent::new(vec![]),
            rule: Rule::Axiom,
            premises: vec![],
        };
        assert_eq!(axiom.cut_count(), 0);
        assert!(axiom.is_cut_free());
    }

    #[test]
    fn test_depth() {
        let leaf = Proof {
            conclusion: Sequent::new(vec![]),
            rule: Rule::Axiom,
            premises: vec![],
        };
        assert_eq!(leaf.depth(), 1);

        let with_premise = Proof {
            conclusion: Sequent::new(vec![]),
            rule: Rule::BottomIntro,
            premises: vec![leaf],
        };
        assert_eq!(with_premise.depth(), 2);
    }
}
