use crate::core::pokemove::{move_name::MoveName, pokemove::PokeMove};

#[derive(Clone, Copy)]
pub struct MoveContext {
    pub src_trainer: bool,
    pub target_trainer: bool,
    pub move_name: MoveName,
    pub pokemove: &'static PokeMove,
}
