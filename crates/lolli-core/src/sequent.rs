//! Sequent representation for linear logic.
//!
//! This module provides sequent data structures for both one-sided and two-sided sequents.

use crate::Formula;

/// A one-sided sequent ⊢ Γ.
///
/// We use a zone-based representation for focused proof search:
/// - Linear zone: formulas that must be used exactly once
/// - Unrestricted zone: formulas under ? that can be reused
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Sequent {
    /// Linear hypotheses (multiset, represented as Vec)
    pub linear: Vec<Formula>,

    /// Unrestricted hypotheses (under ?)
    pub unrestricted: Vec<Formula>,

    /// Currently focused formula (if any)
    pub focus: Option<Formula>,
}

impl Sequent {
    /// Create a new sequent from a list of formulas.
    pub fn new(formulas: Vec<Formula>) -> Self {
        Sequent {
            linear: formulas,
            unrestricted: vec![],
            focus: None,
        }
    }

    /// Check if sequent is empty (proven).
    pub fn is_empty(&self) -> bool {
        self.linear.is_empty() && self.focus.is_none()
    }

    /// Focus on a formula at the given index.
    pub fn focus_on(&self, idx: usize) -> Option<Sequent> {
        if idx >= self.linear.len() {
            return None;
        }

        let mut new_linear = self.linear.clone();
        let focused = new_linear.remove(idx);

        Some(Sequent {
            linear: new_linear,
            unrestricted: self.unrestricted.clone(),
            focus: Some(focused),
        })
    }

    /// Unfocus, returning formula to linear zone.
    pub fn unfocus(&self) -> Sequent {
        let mut new_linear = self.linear.clone();
        if let Some(f) = &self.focus {
            new_linear.push(f.clone());
        }

        Sequent {
            linear: new_linear,
            unrestricted: self.unrestricted.clone(),
            focus: None,
        }
    }

    /// Pretty print the sequent.
    pub fn pretty(&self) -> String {
        let formulas: Vec<String> = self.linear.iter().map(|f| f.pretty()).collect();
        format!("⊢ {}", formulas.join(", "))
    }
}

/// A two-sided sequent Γ ⊢ Δ (for user-facing API).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TwoSidedSequent {
    /// Left side of the turnstile (antecedent)
    pub antecedent: Vec<Formula>,
    /// Right side of the turnstile (succedent)
    pub succedent: Vec<Formula>,
}

impl TwoSidedSequent {
    /// Create a new two-sided sequent.
    pub fn new(antecedent: Vec<Formula>, succedent: Vec<Formula>) -> Self {
        TwoSidedSequent {
            antecedent,
            succedent,
        }
    }

    /// Convert to one-sided: Γ ⊢ Δ becomes ⊢ Γ⊥, Δ
    pub fn to_one_sided(&self) -> Sequent {
        let mut formulas: Vec<Formula> = self.antecedent.iter().map(|f| f.negate()).collect();
        formulas.extend(self.succedent.clone());

        Sequent::new(formulas)
    }

    /// Pretty print the sequent.
    pub fn pretty(&self) -> String {
        let left: Vec<String> = self.antecedent.iter().map(|f| f.pretty()).collect();
        let right: Vec<String> = self.succedent.iter().map(|f| f.pretty()).collect();
        format!("{} ⊢ {}", left.join(", "), right.join(", "))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sided_to_one_sided() {
        let seq = TwoSidedSequent::new(
            vec![Formula::Atom("A".to_string())],
            vec![Formula::Atom("B".to_string())],
        );

        let one_sided = seq.to_one_sided();
        assert_eq!(one_sided.linear.len(), 2);
        assert_eq!(one_sided.linear[0], Formula::NegAtom("A".to_string()));
        assert_eq!(one_sided.linear[1], Formula::Atom("B".to_string()));
    }

    #[test]
    fn test_focus_unfocus() {
        let seq = Sequent::new(vec![
            Formula::Atom("A".to_string()),
            Formula::Atom("B".to_string()),
        ]);

        let focused = seq.focus_on(0).unwrap();
        assert_eq!(focused.focus, Some(Formula::Atom("A".to_string())));
        assert_eq!(focused.linear.len(), 1);

        let unfocused = focused.unfocus();
        assert_eq!(unfocused.linear.len(), 2);
        assert!(unfocused.focus.is_none());
    }
}
