use std::path::PathBuf;

fn main() {
    let dir: PathBuf = PathBuf::from("src");

    cc::Build::new()
        .include(dir.join("tree-sitter"))
        .include(&dir)
        .file(dir.join("parser.c"))
        .compile("tree-sitter-sexp");
}
