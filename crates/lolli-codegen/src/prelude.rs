//! Prelude types for generated Rust code.
//!
//! These types implement the linear logic connectives in Rust.

/// Prelude code to include in generated modules.
///
/// This includes type definitions for linear logic connectives
/// that don't have direct Rust equivalents.
pub const PRELUDE: &str = r#"
// ============================================================================
// Linear Logic Prelude
// ============================================================================

/// Either type for sum (A ⊕ B)
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Either<A, B> {
    Left(A),
    Right(B),
}

impl<A, B> Either<A, B> {
    /// Check if this is the left variant.
    pub fn is_left(&self) -> bool {
        matches!(self, Either::Left(_))
    }

    /// Check if this is the right variant.
    pub fn is_right(&self) -> bool {
        matches!(self, Either::Right(_))
    }

    /// Unwrap the left variant.
    pub fn unwrap_left(self) -> A {
        match self {
            Either::Left(a) => a,
            Either::Right(_) => panic!("called unwrap_left on Right"),
        }
    }

    /// Unwrap the right variant.
    pub fn unwrap_right(self) -> B {
        match self {
            Either::Left(_) => panic!("called unwrap_right on Left"),
            Either::Right(b) => b,
        }
    }
}

/// With type for lazy pair (A & B)
///
/// Unlike a tuple, you can only observe one component.
/// This implements additive conjunction.
#[derive(Clone)]
pub struct With<A, B> {
    left: Box<dyn FnOnce() -> A>,
    right: Box<dyn FnOnce() -> B>,
}

impl<A: 'static, B: 'static> With<A, B> {
    /// Create a new With from two thunks.
    pub fn new<F, G>(left: F, right: G) -> Self
    where
        F: FnOnce() -> A + 'static,
        G: FnOnce() -> B + 'static,
    {
        With {
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    /// Project to the left component.
    pub fn fst(self) -> A {
        (self.left)()
    }

    /// Project to the right component.
    pub fn snd(self) -> B {
        (self.right)()
    }
}

/// Top type (unit for &)
///
/// Always available, provides no information.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Top;

/// Void type (zero, empty type for ⊕)
///
/// Cannot be constructed, represents impossibility.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Void {}

impl Void {
    /// Since Void is uninhabited, this is unreachable.
    pub fn absurd<T>(self) -> T {
        match self {}
    }
}

/// Par type (A ⅋ B) - dual of tensor
///
/// Represents a continuation that expects either A or B.
/// In terms of linear logic session types, this is parallel composition.
pub struct Par<A, B> {
    _phantom: std::marker::PhantomData<(A, B)>,
}

/// Demand type (?A) - controlled use of a replicable resource
///
/// Represents a demand for a value that may be copied or discarded.
pub struct Demand<A> {
    value: std::rc::Rc<A>,
}

impl<A> Demand<A> {
    /// Create a new demand.
    pub fn new(value: A) -> Self {
        Demand {
            value: std::rc::Rc::new(value),
        }
    }

    /// Access the value.
    pub fn get(&self) -> &A {
        &self.value
    }

    /// Clone the demand.
    pub fn duplicate(&self) -> Self {
        Demand {
            value: std::rc::Rc::clone(&self.value),
        }
    }
}

impl<A: Clone> Demand<A> {
    /// Extract a copy of the value.
    pub fn extract(&self) -> A {
        (*self.value).clone()
    }
}
"#;

#[cfg(test)]
mod tests {
    #[test]
    fn test_prelude_not_empty() {
        assert!(!super::PRELUDE.is_empty());
    }

    #[test]
    fn test_prelude_contains_types() {
        assert!(super::PRELUDE.contains("Either"));
        assert!(super::PRELUDE.contains("With"));
        assert!(super::PRELUDE.contains("Top"));
        assert!(super::PRELUDE.contains("Void"));
        assert!(super::PRELUDE.contains("Par"));
        assert!(super::PRELUDE.contains("Demand"));
    }
}
