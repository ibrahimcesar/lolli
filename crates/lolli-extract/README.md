# lolli-extract

Term extraction for the Lolli linear logic workbench.

Extracts computational content (lambda terms) from proofs via the Curry-Howard correspondence.

## Curry-Howard Correspondence

| Logic | Term |
|-------|------|
| A ⊗ B | Pair |
| A ⊸ B | Function |
| A & B | Lazy pair |
| A ⊕ B | Sum (Either) |
| !A | Copyable value |

## Usage

```rust
use lolli_extract::Extractor;

let mut extractor = Extractor::new();
let term = extractor.extract(&proof);
let normalized = term.normalize();
```

## Part of Lolli

This is part of the [Lolli](https://github.com/ibrahimcesar/lolli) linear logic workbench.

## License

MIT
