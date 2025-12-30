use crate::core::poketype::poketype::PokeType;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub enum PokemonTyping {
    MonoType(PokeType),
    DualType(PokeType, PokeType),
}