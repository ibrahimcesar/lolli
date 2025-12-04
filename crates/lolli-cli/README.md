# lolli-cli

Command-line interface for the Lolli linear logic workbench.

## Installation

```bash
cargo install lolli-cli
```

## Usage

```bash
# Parse a formula
lolli parse "A -o B"

# Prove a sequent
lolli prove "A, B |- A * B"

# Extract a term
lolli extract "A |- A"

# Generate Rust code
lolli codegen "FileHandle |- Contents * ClosedHandle"

# Visualize a proof
lolli viz "A |- A" --format latex

# Interactive REPL
lolli repl
```

## Commands

- `parse` - Parse and pretty-print a formula
- `prove` - Check if a sequent is provable
- `extract` - Extract a term from a proof
- `codegen` - Generate Rust code from a proof
- `viz` - Visualize a proof
- `repl` - Interactive mode

## Part of Lolli

This is part of the [Lolli](https://github.com/ibrahimcesar/lolli) linear logic workbench.

## License

MIT
