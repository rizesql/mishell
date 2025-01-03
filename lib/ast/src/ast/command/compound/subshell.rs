use super::CompoundBlock;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Subshell(CompoundBlock);

impl std::fmt::Display for Subshell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "( {} )", self.0)
    }
}
