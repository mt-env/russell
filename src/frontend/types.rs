use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Spanned<T> {
    pub offset: usize,
    pub node: T,
}

impl<T: Display> Display for Spanned<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.node)
    }
}
