use crate::core::{pokemon::stat_enum::StatEnum, poketype::pokemon_typing::PokemonTyping};

pub struct BasePokemon {
    pub hp: u16,
    pub attack: u16,
    pub spattack: u16,
    pub defense: u16,
    pub spdefense: u16,
    pub speed: u16,

    pub typing: PokemonTyping,
}

impl BasePokemon {
    pub fn stat_for_enum(&self, stat_enum: &StatEnum) -> u16 {
        match stat_enum {
            StatEnum::HP => self.hp,
            StatEnum::Attack => self.attack,
            StatEnum::SpecialAttack => self.spattack,
            StatEnum::Defense => self.defense,
            StatEnum::SpecialDefense => self.spdefense,
            StatEnum::Speed => self.speed,
        }
    }
}