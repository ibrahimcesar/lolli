//! Rust code generation from linear lambda terms.
//!
//! This module translates extracted terms into Rust code.

use lolli_core::{Formula, Term, TwoSidedSequent};
use crate::types::TypeGenerator;

/// Rust code generator.
///
/// Translates linear lambda terms into Rust code, preserving
/// linear ownership semantics through Rust's move semantics.
pub struct RustCodegen {
    /// Current indentation level
    indent: usize,
    /// Type generator
    types: TypeGenerator,
    /// Variable counter for fresh names
    var_counter: usize,
}

impl Default for RustCodegen {
    fn default() -> Self {
        Self::new()
    }
}

impl RustCodegen {
    /// Create a new code generator.
    pub fn new() -> Self {
        Self {
            indent: 0,
            types: TypeGenerator::new(),
            var_counter: 0,
        }
    }

    /// Generate a fresh variable name.
    pub fn fresh_var(&mut self) -> String {
        let v = format!("_v{}", self.var_counter);
        self.var_counter += 1;
        v
    }

    /// Get current indentation string.
    fn indent_str(&self) -> String {
        "    ".repeat(self.indent)
    }

    /// Generate a Rust type from a formula.
    pub fn formula_to_type(&self, formula: &Formula) -> String {
        self.types.generate(formula)
    }

    /// Generate Rust code from a term.
    pub fn term_to_code(&mut self, term: &Term) -> String {
        match term {
            Term::Var(v) => v.clone(),

            Term::Unit => "()".to_string(),

            Term::Trivial => "Top".to_string(),

            Term::Pair(a, b) => {
                format!("({}, {})", self.term_to_code(a), self.term_to_code(b))
            }

            Term::LetPair(x, y, pair, body) => {
                let pair_code = self.term_to_code(pair);
                let body_code = self.term_to_code(body);
                format!("{{ let ({}, {}) = {}; {} }}", x, y, pair_code, body_code)
            }

            Term::Abs(x, body) => {
                let body_code = self.term_to_code(body);
                format!("|{}| {}", x, body_code)
            }

            Term::App(f, a) => {
                let f_code = self.term_to_code(f);
                let a_code = self.term_to_code(a);
                // Handle the case where f is a closure
                if matches!(f.as_ref(), Term::Abs(_, _)) {
                    format!("({})({})", f_code, a_code)
                } else {
                    format!("{}({})", f_code, a_code)
                }
            }

            Term::Inl(a) => {
                format!("Either::Left({})", self.term_to_code(a))
            }

            Term::Inr(b) => {
                format!("Either::Right({})", self.term_to_code(b))
            }

            Term::Case(scrut, x, left, y, right) => {
                let scrut_code = self.term_to_code(scrut);
                let left_code = self.term_to_code(left);
                let right_code = self.term_to_code(right);
                format!(
                    "match {} {{ Either::Left({}) => {}, Either::Right({}) => {} }}",
                    scrut_code, x, left_code, y, right_code
                )
            }

            Term::Fst(p) => {
                let p_code = self.term_to_code(p);
                format!("{}.0", p_code)
            }

            Term::Snd(p) => {
                let p_code = self.term_to_code(p);
                format!("{}.1", p_code)
            }

            Term::Abort(e) => {
                let e_code = self.term_to_code(e);
                format!("match {} {{}}", e_code)
            }

            Term::Promote(a) => {
                format!("Rc::new({})", self.term_to_code(a))
            }

            Term::Derelict(e) => {
                let e_code = self.term_to_code(e);
                format!("Rc::try_unwrap({}).unwrap_or_else(|rc| (*rc).clone())", e_code)
            }

            Term::Discard(_, body) => {
                // In Rust, dropping is implicit
                // We could add drop(_discarded) but it's not necessary
                self.term_to_code(body)
            }

            Term::Copy(src, x, y, body) => {
                let src_code = self.term_to_code(src);
                let body_code = self.term_to_code(body);
                format!(
                    "{{ let {} = Rc::clone(&{}); let {} = {}; {} }}",
                    x, src_code, y, src_code, body_code
                )
            }
        }
    }

    /// Generate a complete function from a sequent and term.
    pub fn generate_function(&mut self, name: &str, sequent: &TwoSidedSequent, term: &Term) -> String {
        let mut lines = Vec::new();

        // Generate function signature
        let mut args = Vec::new();
        for (i, formula) in sequent.antecedent.iter().enumerate() {
            let arg_name = format!("arg{}", i);
            let arg_type = self.types.generate(formula);
            args.push(format!("{}: {}", arg_name, arg_type));
        }

        let return_type = self.types.generate_return(&sequent.succedent);

        lines.push(format!(
            "fn {}({}) -> {} {{",
            name,
            args.join(", "),
            return_type
        ));

        // Generate function body
        self.indent += 1;
        let body = self.term_to_code(term);
        lines.push(format!("{}{}", self.indent_str(), body));
        self.indent -= 1;

        lines.push("}".to_string());

        lines.join("\n")
    }

    /// Generate a complete module with necessary imports and type definitions.
    pub fn generate_module(&mut self, name: &str, sequent: &TwoSidedSequent, term: &Term) -> String {
        let mut lines = Vec::new();

        // Module documentation
        lines.push(format!("//! Generated from sequent: {}", sequent.pretty()));
        lines.push("//!".to_string());
        lines.push("//! This code was generated by lolli-codegen from a linear logic proof.".to_string());
        lines.push("".to_string());

        // Imports
        lines.push("use std::rc::Rc;".to_string());
        lines.push("".to_string());

        // Include prelude types
        lines.push(crate::PRELUDE.to_string());
        lines.push("".to_string());

        // Generate the main function
        lines.push(self.generate_function(name, sequent, term));

        lines.join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_var() {
        let mut codegen = RustCodegen::new();
        let term = Term::Var("x".to_string());
        assert_eq!(codegen.term_to_code(&term), "x");
    }

    #[test]
    fn test_unit() {
        let mut codegen = RustCodegen::new();
        assert_eq!(codegen.term_to_code(&Term::Unit), "()");
    }

    #[test]
    fn test_pair() {
        let mut codegen = RustCodegen::new();
        let term = Term::Pair(
            Box::new(Term::Var("x".to_string())),
            Box::new(Term::Var("y".to_string())),
        );
        assert_eq!(codegen.term_to_code(&term), "(x, y)");
    }

    #[test]
    fn test_let_pair() {
        let mut codegen = RustCodegen::new();
        let term = Term::LetPair(
            "a".to_string(),
            "b".to_string(),
            Box::new(Term::Var("p".to_string())),
            Box::new(Term::Var("a".to_string())),
        );
        assert_eq!(codegen.term_to_code(&term), "{ let (a, b) = p; a }");
    }

    #[test]
    fn test_abs() {
        let mut codegen = RustCodegen::new();
        let term = Term::Abs(
            "x".to_string(),
            Box::new(Term::Var("x".to_string())),
        );
        assert_eq!(codegen.term_to_code(&term), "|x| x");
    }

    #[test]
    fn test_app() {
        let mut codegen = RustCodegen::new();
        let term = Term::App(
            Box::new(Term::Var("f".to_string())),
            Box::new(Term::Var("x".to_string())),
        );
        assert_eq!(codegen.term_to_code(&term), "f(x)");
    }

    #[test]
    fn test_inl_inr() {
        let mut codegen = RustCodegen::new();

        let inl = Term::Inl(Box::new(Term::Var("x".to_string())));
        assert_eq!(codegen.term_to_code(&inl), "Either::Left(x)");

        let inr = Term::Inr(Box::new(Term::Var("y".to_string())));
        assert_eq!(codegen.term_to_code(&inr), "Either::Right(y)");
    }

    #[test]
    fn test_case() {
        let mut codegen = RustCodegen::new();
        let term = Term::Case(
            Box::new(Term::Var("e".to_string())),
            "x".to_string(),
            Box::new(Term::Var("x".to_string())),
            "y".to_string(),
            Box::new(Term::Var("y".to_string())),
        );
        assert_eq!(
            codegen.term_to_code(&term),
            "match e { Either::Left(x) => x, Either::Right(y) => y }"
        );
    }

    #[test]
    fn test_fst_snd() {
        let mut codegen = RustCodegen::new();

        let fst = Term::Fst(Box::new(Term::Var("p".to_string())));
        assert_eq!(codegen.term_to_code(&fst), "p.0");

        let snd = Term::Snd(Box::new(Term::Var("p".to_string())));
        assert_eq!(codegen.term_to_code(&snd), "p.1");
    }

    #[test]
    fn test_promote() {
        let mut codegen = RustCodegen::new();
        let term = Term::Promote(Box::new(Term::Var("x".to_string())));
        assert_eq!(codegen.term_to_code(&term), "Rc::new(x)");
    }

    #[test]
    fn test_copy() {
        let mut codegen = RustCodegen::new();
        let term = Term::Copy(
            Box::new(Term::Var("r".to_string())),
            "x".to_string(),
            "y".to_string(),
            Box::new(Term::Pair(
                Box::new(Term::Var("x".to_string())),
                Box::new(Term::Var("y".to_string())),
            )),
        );
        let code = codegen.term_to_code(&term);
        assert!(code.contains("Rc::clone"));
    }

    #[test]
    fn test_generate_function() {
        let mut codegen = RustCodegen::new();
        let sequent = TwoSidedSequent::new(
            vec![Formula::atom("A"), Formula::atom("B")],
            vec![Formula::tensor(Formula::atom("A"), Formula::atom("B"))],
        );
        let term = Term::Pair(
            Box::new(Term::Var("arg0".to_string())),
            Box::new(Term::Var("arg1".to_string())),
        );

        let code = codegen.generate_function("make_pair", &sequent, &term);
        assert!(code.contains("fn make_pair(arg0: A, arg1: B) -> (A, B)"));
        assert!(code.contains("(arg0, arg1)"));
    }
}
