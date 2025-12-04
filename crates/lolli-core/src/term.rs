//! Lambda term representation.
//!
//! This module provides the [`Term`] enum representing linear lambda terms
//! extracted from proofs via the Curry-Howard correspondence.

use std::collections::HashSet;

/// Linear λ-terms extracted from proofs.
///
/// These terms correspond to the computational content of linear logic proofs
/// via the Curry-Howard correspondence.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Term {
    // Variables
    /// Variable reference
    Var(String),

    // Multiplicatives
    /// Unit value ()
    Unit,
    /// Pair (a, b)
    Pair(Box<Term>, Box<Term>),
    /// Let pair: let (x, y) = e in e'
    LetPair(String, String, Box<Term>, Box<Term>),

    // Linear functions
    /// Abstraction: λx. e
    Abs(String, Box<Term>),
    /// Application: e e'
    App(Box<Term>, Box<Term>),

    // Additives
    /// Left injection: inl e
    Inl(Box<Term>),
    /// Right injection: inr e
    Inr(Box<Term>),
    /// Case analysis: case e of inl x => e1 | inr y => e2
    Case(Box<Term>, String, Box<Term>, String, Box<Term>),
    /// Trivial value (unit for &)
    Trivial,
    /// First projection: fst e
    Fst(Box<Term>),
    /// Second projection: snd e
    Snd(Box<Term>),
    /// Abort (for 0): absurd e
    Abort(Box<Term>),

    // Exponentials
    /// Promote: !e (mark as copyable)
    Promote(Box<Term>),
    /// Derelict: use !e as regular e
    Derelict(Box<Term>),
    /// Discard: discard e in e'
    Discard(Box<Term>, Box<Term>),
    /// Copy: copy e as (x, y) in e'
    Copy(Box<Term>, String, String, Box<Term>),
}

impl Term {
    /// Compute the free variables of this term.
    pub fn free_vars(&self) -> HashSet<String> {
        match self {
            Term::Var(v) => {
                let mut set = HashSet::new();
                set.insert(v.clone());
                set
            }
            Term::Unit | Term::Trivial => HashSet::new(),
            Term::Pair(a, b) | Term::App(a, b) => {
                let mut fv = a.free_vars();
                fv.extend(b.free_vars());
                fv
            }
            Term::LetPair(x, y, pair, body) => {
                let mut fv = pair.free_vars();
                let mut body_fv = body.free_vars();
                body_fv.remove(x);
                body_fv.remove(y);
                fv.extend(body_fv);
                fv
            }
            Term::Abs(x, body) => {
                let mut fv = body.free_vars();
                fv.remove(x);
                fv
            }
            Term::Inl(e) | Term::Inr(e) | Term::Fst(e) | Term::Snd(e) | Term::Abort(e) => {
                e.free_vars()
            }
            Term::Case(scrut, x, left, y, right) => {
                let mut fv = scrut.free_vars();
                let mut left_fv = left.free_vars();
                left_fv.remove(x);
                let mut right_fv = right.free_vars();
                right_fv.remove(y);
                fv.extend(left_fv);
                fv.extend(right_fv);
                fv
            }
            Term::Promote(e) | Term::Derelict(e) => e.free_vars(),
            Term::Discard(discarded, body) => {
                let mut fv = discarded.free_vars();
                fv.extend(body.free_vars());
                fv
            }
            Term::Copy(src, x, y, body) => {
                let mut fv = src.free_vars();
                let mut body_fv = body.free_vars();
                body_fv.remove(x);
                body_fv.remove(y);
                fv.extend(body_fv);
                fv
            }
        }
    }

    /// Substitute a term for a variable.
    pub fn substitute(&self, var: &str, replacement: &Term) -> Term {
        match self {
            Term::Var(v) if v == var => replacement.clone(),
            Term::Var(v) => Term::Var(v.clone()),
            Term::Unit => Term::Unit,
            Term::Trivial => Term::Trivial,
            Term::Pair(a, b) => Term::Pair(
                Box::new(a.substitute(var, replacement)),
                Box::new(b.substitute(var, replacement)),
            ),
            Term::LetPair(x, y, pair, body) => {
                let new_pair = pair.substitute(var, replacement);
                let new_body = if x == var || y == var {
                    body.as_ref().clone()
                } else {
                    body.substitute(var, replacement)
                };
                Term::LetPair(x.clone(), y.clone(), Box::new(new_pair), Box::new(new_body))
            }
            Term::Abs(x, body) if x == var => Term::Abs(x.clone(), body.clone()),
            Term::Abs(x, body) => {
                Term::Abs(x.clone(), Box::new(body.substitute(var, replacement)))
            }
            Term::App(f, a) => Term::App(
                Box::new(f.substitute(var, replacement)),
                Box::new(a.substitute(var, replacement)),
            ),
            Term::Inl(e) => Term::Inl(Box::new(e.substitute(var, replacement))),
            Term::Inr(e) => Term::Inr(Box::new(e.substitute(var, replacement))),
            Term::Case(scrut, x, left, y, right) => {
                let new_scrut = scrut.substitute(var, replacement);
                let new_left = if x == var {
                    left.as_ref().clone()
                } else {
                    left.substitute(var, replacement)
                };
                let new_right = if y == var {
                    right.as_ref().clone()
                } else {
                    right.substitute(var, replacement)
                };
                Term::Case(
                    Box::new(new_scrut),
                    x.clone(),
                    Box::new(new_left),
                    y.clone(),
                    Box::new(new_right),
                )
            }
            Term::Fst(e) => Term::Fst(Box::new(e.substitute(var, replacement))),
            Term::Snd(e) => Term::Snd(Box::new(e.substitute(var, replacement))),
            Term::Abort(e) => Term::Abort(Box::new(e.substitute(var, replacement))),
            Term::Promote(e) => Term::Promote(Box::new(e.substitute(var, replacement))),
            Term::Derelict(e) => Term::Derelict(Box::new(e.substitute(var, replacement))),
            Term::Discard(discarded, body) => Term::Discard(
                Box::new(discarded.substitute(var, replacement)),
                Box::new(body.substitute(var, replacement)),
            ),
            Term::Copy(src, x, y, body) => {
                let new_src = src.substitute(var, replacement);
                let new_body = if x == var || y == var {
                    body.as_ref().clone()
                } else {
                    body.substitute(var, replacement)
                };
                Term::Copy(Box::new(new_src), x.clone(), y.clone(), Box::new(new_body))
            }
        }
    }

    /// Pretty print the term.
    pub fn pretty(&self) -> String {
        match self {
            Term::Var(v) => v.clone(),
            Term::Unit => "()".to_string(),
            Term::Trivial => "⟨⟩".to_string(),
            Term::Pair(a, b) => format!("({}, {})", a.pretty(), b.pretty()),
            Term::LetPair(x, y, pair, body) => {
                format!("let ({}, {}) = {} in {}", x, y, pair.pretty(), body.pretty())
            }
            Term::Abs(x, body) => format!("λ{}. {}", x, body.pretty()),
            Term::App(f, a) => format!("({} {})", f.pretty(), a.pretty()),
            Term::Inl(e) => format!("inl {}", e.pretty()),
            Term::Inr(e) => format!("inr {}", e.pretty()),
            Term::Case(scrut, x, left, y, right) => {
                format!(
                    "case {} of {{ inl {} => {} | inr {} => {} }}",
                    scrut.pretty(),
                    x,
                    left.pretty(),
                    y,
                    right.pretty()
                )
            }
            Term::Fst(e) => format!("fst {}", e.pretty()),
            Term::Snd(e) => format!("snd {}", e.pretty()),
            Term::Abort(e) => format!("absurd {}", e.pretty()),
            Term::Promote(e) => format!("!{}", e.pretty()),
            Term::Derelict(e) => format!("derelict {}", e.pretty()),
            Term::Discard(_, body) => format!("discard in {}", body.pretty()),
            Term::Copy(src, x, y, body) => {
                format!("copy {} as ({}, {}) in {}", src.pretty(), x, y, body.pretty())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_free_vars() {
        let t = Term::Abs(
            "x".to_string(),
            Box::new(Term::App(
                Box::new(Term::Var("x".to_string())),
                Box::new(Term::Var("y".to_string())),
            )),
        );
        let fv = t.free_vars();
        assert!(!fv.contains("x"));
        assert!(fv.contains("y"));
    }

    #[test]
    fn test_substitute() {
        let t = Term::Var("x".to_string());
        let result = t.substitute("x", &Term::Unit);
        assert_eq!(result, Term::Unit);
    }
}
