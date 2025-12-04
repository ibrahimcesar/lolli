# lolli-core

Core data structures for the Lolli linear logic workbench.

This crate provides the fundamental types for working with linear logic:

- `Formula` - Linear logic formula representation (⊗, ⅋, ⊸, &, ⊕, !, ?, etc.)
- `Sequent` - Sequent representation for proof search
- `Proof` - Proof tree data structures
- `Term` - Lambda terms extracted from proofs

## Usage

```rust
use lolli_core::{Formula, Sequent};

// Create a linear implication: A ⊸ B
let formula = Formula::Lolli(
    Box::new(Formula::Atom("A".into())),
    Box::new(Formula::Atom("B".into())),
);

println!("{}", formula.pretty()); // (A ⊸ B)
```

## Part of Lolli

This is part of the [Lolli](https://github.com/ibrahimcesar/lolli) linear logic workbench - a toolkit for parsing, proving, extracting, and compiling linear logic formulas to Rust.

## License

MIT
