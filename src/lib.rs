use anyhow::{anyhow, Context, Error};
use std::fmt;

mod ffi;

#[derive(Clone, Debug)]
pub enum Sexp {
    Atom(String),
    List(Vec<Sexp>),
    Nil,
}

impl Sexp {
    pub fn of_str(input: &str) -> Result<Sexp, Error> {
        let mut parser = ffi::parser();
        let tree = parser
            .parse(input, None)
            .context("Could not parse anything")?;
        let root = tree.root_node();

        let mut walker = root.walk();
        walker.goto_first_child(); // we skip the top-level `sexp` node
        Sexp::build_tree(walker.node(), input.as_bytes())
    }

    fn build_tree(root: tree_sitter::Node, bytes: &[u8]) -> Result<Sexp, Error> {
        match root.kind() {
            "atom" => {
                let text = root.utf8_text(&bytes)?.to_string();
                Ok(Sexp::Atom(text))
            }
            kind @ "list" | kind @ "ERROR" | kind @ "MISSING" => {
                let mut walker = root.walk();
                walker.goto_first_child();
                let mut children = match kind {
                    "list" => vec![],
                    _ => vec![Sexp::Atom(kind.to_string())],
                };
                while walker.goto_next_sibling() {
                    let child = walker.node();
                    children.push(Sexp::build_tree(child, bytes)?);
                }
                Ok(Sexp::List(children))
            }
            ")" => Ok(Sexp::Nil),
            kind => Err(anyhow!("Unknown node kind {:?}", kind)),
        }
    }

    pub fn size(&self) -> u32 {
        match self {
            Sexp::Nil => 0,
            Sexp::Atom(s) => s.len() as u32,
            Sexp::List(parts) => {
                let mut s = 0;
                for p in parts {
                    s += p.size();
                }
                s
            }
        }
    }
}

#[derive(Debug)]
pub struct PrettyPrinter {
    max_width: u32,
    current_width: u32,
    indent_size: u32,
    current_depth: u32,
}

impl PrettyPrinter {
    pub fn new() -> PrettyPrinter {
        PrettyPrinter {
            current_depth: 0,
            max_width: 150,
            current_width: 0,
            indent_size: 1,
        }
    }

    fn padding(&self) -> u32 {
        if self.current_depth == 0 {
            0
        } else {
            (self.current_depth - 1) * self.indent_size
        }
    }

    pub fn pp(&mut self, sexp: &Sexp, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match sexp {
            Sexp::Atom(atom) => {
                self.current_width += atom.len() as u32;
                write!(fmt, "{}", atom)
            }
            Sexp::Nil => {
                self.current_depth -= 1;
                Ok(())
            }
            Sexp::List(parts) if parts.len() > 0 => {
                self.current_depth += 1;
                let next_term_width = self.current_width + self.padding() + sexp.size();
                let term_overflows = next_term_width > self.max_width / 2;

                if term_overflows && self.current_depth > 1 {
                    self.current_width = self.padding();
                }

                write!(fmt, "(")?;
                self.pp(&parts[0], fmt)?;

                for p in parts[1..].iter() {
                    match p {
                        Sexp::Nil => {
                            self.pp(&p, fmt)?;
                        }
                        _ => {
                            let part_size = next_term_width + self.padding() + p.size();
                            let part_will_overflow = part_size > self.max_width;
                            if part_will_overflow {
                                write!(fmt, "\n")?;
                                for _ in 0..(self.padding() + self.indent_size) {
                                    write!(fmt, " ")?
                                }
                                self.pp(&p, fmt)?;
                            } else {
                                write!(fmt, " ")?;
                                self.pp(&p, fmt)?;
                            }
                        }
                    }
                }
                write!(fmt, ")")?;

                Ok(())
            }
            Sexp::List(_) => write!(fmt, "()"),
        }
    }
}

impl fmt::Display for Sexp {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        PrettyPrinter::new().pp(self, fmt)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_sexpr() {
        assert_eq!(Sexp::of_str(&"(sexp (").is_err(), true,);
    }

    #[test]
    fn test_single_sexpr() {
        assert_eq!(
            Sexp::of_str(&"source_file").unwrap().to_string(),
            r#"source_file"#.trim().to_string()
        );
    }

    #[test]
    fn test_sibling_sexpr() {
        assert_eq!(
            Sexp::of_str(&"(source file)").unwrap().to_string(),
            r#"(source file)"#.trim().to_string()
        );
        assert_eq!(
            Sexp::of_str(&"(source file tree)").unwrap().to_string(),
            r#"(source file tree)"#.trim().to_string()
        );
    }

    #[test]
    fn test_nested_sexpr() {
        assert_eq!(
            Sexp::of_str(&"(source (file))").unwrap().to_string(),
            r#"(source (file))"#.trim().to_string()
        );
    }

    #[test]
    fn test_nested_sibling_sexpr() {
        assert_eq!(
            Sexp::of_str(&"(source (file tree))").unwrap().to_string(),
            r#"(source (file tree))"#.trim().to_string()
        );
    }

    #[test]
    fn test_field_sexpr() {
        assert_eq!(
            Sexp::of_str(&"(source file: test)").unwrap().to_string(),
            r#"(source file: test)"#.trim().to_string()
        );
    }

    #[test]
    fn test_real_life_sexpr() {
        let sexp = Sexp::of_str(include_str!("./big_fixture.in.sexp")).unwrap();
        println!("{}", sexp);
        assert_eq!(
            sexp.to_string(),
            include_str!("./big_fixture.out.sexp").trim().to_string()
        );
    }

    #[test]
    fn test_pretty_printing_sexpr() {
        let sexp = Sexp::of_str(
            &"



(source_file    
  (expression
    (function_call
      (qualified_function_name
        (expression
          (term
            (atom
              (unquoted_atom))))
        (atom
          (unquoted_atom)))
      (expression
        (term
          (integer)))
      (expression
        (function_call
          (qualified_function_name
            (expression
              (term
                (atom
                  (unquoted_atom))))
            (atom
              (unquoted_atom)))
          (expression
            (term
              (integer)))
          (expression
            (term
              (integer)))
          (expression   
            (term
              (integer))))))))

",
        )
        .unwrap();
        println!("{}", sexp);
        assert_eq!(
            sexp.to_string(),
            r#"(source_file
 (expression
  (function_call
   (qualified_function_name (expression (term (atom (unquoted_atom)))) (atom (unquoted_atom)))
   (expression (term (integer)))
   (expression
    (function_call
     (qualified_function_name (expression (term (atom (unquoted_atom)))) (atom (unquoted_atom)))
     (expression (term (integer)))
     (expression (term (integer)))
     (expression (term (integer))))))))"#
                .to_string()
        );
    }
}
