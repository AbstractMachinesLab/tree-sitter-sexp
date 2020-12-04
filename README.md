# tree-sitter grammar for S-expressions (+ Rust bindings)

## Getting Started

You can add this lib as a dependency:

```toml
[dependencies]
tree-sitter-sexp = { git = "https://github.com/AbstractMachinesLab/tree-sitter-sexp" }
```

And then you can parse strings with the `Sexp` struct like this:

```rust
let sexprs = Sexp::of_str("(hello (world))")?;

sexprs.to_string();
// (hello (world))
```
