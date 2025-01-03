use crate::ast;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct For {
    lhs: String,
    rhs: Option<Vec<ast::Word>>,
    block: ast::command::compound::DoScope,
}

impl std::fmt::Display for For {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "for {} in ", self.lhs)?;

        if let Some(rhs) = &self.rhs {
            let mut iter = rhs.iter();

            if let Some(fst) = iter.next() {
                write!(f, "{fst}")?;

                for it in iter {
                    write!(f, " {it}")?;
                }
            }
        }

        writeln!(f, ";")?;
        write!(f, "{}", self.block)
    }
}
