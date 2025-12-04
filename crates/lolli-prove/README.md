# lolli-prove

Proof search for the Lolli linear logic workbench.

Implements focused proof search for MALL (Multiplicative-Additive Linear Logic) and MELL (with exponentials).

## Features

- Focused sequent calculus (Andreoli, 1992)
- Efficient proof search with caching
- Support for MALL and MELL fragments

## Usage

```rust
use lolli_prove::FocusedProver;
use lolli_core::Sequent;

let mut prover = FocusedProver::new(100);
let sequent = /* ... */;

if let Some(proof) = prover.prove(&sequent) {
    println!("Provable!");
}
```

## Part of Lolli

This is part of the [Lolli](https://github.com/ibrahimcesar/lolli) linear logic workbench.

## License

MIT
