# lolli-codegen

Code generation for the Lolli linear logic workbench.

Generates Rust code from linear logic proofs, enforcing resource invariants at compile time.

## Type Mapping

| Linear Logic | Rust Type |
|--------------|-----------|
| A ⊗ B | `(A, B)` |
| A ⊸ B | `impl FnOnce(A) -> B` |
| A & B | `With<A, B>` |
| A ⊕ B | `Either<A, B>` |
| !A | `Rc<A>` |
| 1 | `()` |
| 0 | `!` (never) |

## Usage

```rust
use lolli_codegen::RustCodegen;

let mut codegen = RustCodegen::new();
let rust_code = codegen.generate_module(&proofs);
```

## Part of Lolli

This is part of the [Lolli](https://github.com/ibrahimcesar/lolli) linear logic workbench.

## License

MIT
