# lolli-viz

Visualization for the Lolli linear logic workbench.

Renders proofs as trees, LaTeX, and Graphviz graphs.

## Output Formats

- ASCII/Unicode proof trees
- LaTeX (bussproofs package)
- Graphviz DOT
- SVG (via Graphviz)

## Usage

```rust
use lolli_viz::TreeRenderer;

let renderer = TreeRenderer { unicode: true };
let output = renderer.render(&proof);
```

## Part of Lolli

This is part of the [Lolli](https://github.com/ibrahimcesar/lolli) linear logic workbench.

## License

MIT
