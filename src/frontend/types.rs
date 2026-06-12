#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Spanned<T> {
    pub offset: usize,
    pub node: T,
}
