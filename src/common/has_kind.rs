use std::hash::Hash;

pub trait HasKind {
    type Kind: Copy + Eq + Hash + 'static;

    fn kind(&self) -> Self::Kind;
}
