# lolli-parse

Parser for the Lolli linear logic workbench.

Parses linear logic formulas and sequents from text using a PEG grammar.

## Supported Syntax

| Connective | Unicode | ASCII |
|------------|---------|-------|
| Tensor | ⊗ | * |
| Par | ⅋ | \| |
| Lolli | ⊸ | -o |
| With | & | & |
| Plus | ⊕ | + |
| Bang | ! | ! |
| Why not | ? | ? |
| Negation | A⊥ | A^ |
| Turnstile | ⊢ | \|- |

## Usage

```rust
use lolli_parse::{parse_formula, parse_sequent};

let formula = parse_formula("A -o B").unwrap();
let sequent = parse_sequent("A, B |- A * B").unwrap();
```

## Part of Lolli

This is part of the [Lolli](https://github.com/ibrahimcesar/lolli) linear logic workbench.

## License

MIT
