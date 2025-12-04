//! # lolli-viz
//!
//! Visualization for the Lolli linear logic workbench.
//!
//! This crate provides rendering of proofs as trees, LaTeX, and graphs.

#![warn(missing_docs)]
#![warn(clippy::all)]

pub use lolli_core::Proof;

/// Proof tree renderer.
pub struct TreeRenderer {
    /// Use Unicode box-drawing characters
    pub unicode: bool,
}

impl Default for TreeRenderer {
    fn default() -> Self {
        Self { unicode: true }
    }
}

impl TreeRenderer {
    /// Create a new renderer.
    pub fn new() -> Self {
        Self::default()
    }

    /// Render a proof as ASCII/Unicode text.
    pub fn render(&self, _proof: &Proof) -> String {
        // TODO: Implement in Issue #18
        "/* Proof tree rendering not yet implemented */".to_string()
    }

    /// Render a proof as LaTeX (bussproofs).
    pub fn render_latex(&self, _proof: &Proof) -> String {
        // TODO: Implement in Issue #19
        "% LaTeX rendering not yet implemented".to_string()
    }
}

/// Proof net renderer.
pub struct NetRenderer;

impl NetRenderer {
    /// Render a proof net as Graphviz DOT.
    pub fn render_dot(&self, _proof: &Proof) -> String {
        // TODO: Implement in Issue #20
        "digraph { /* not yet implemented */ }".to_string()
    }
}
