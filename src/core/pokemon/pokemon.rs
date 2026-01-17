use std::collections::{HashMap, HashSet};

use enum_map::EnumMap;
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;

use crate::{
    battle::actions::{Action, MoveSlot},
    core::{
        ability::ability::Ability,
        pokemon::{base_pokemon::BasePokemon, nature::Nature, stat_enum::StatEnum},
        pokemove::move_name::MoveName,
        poketype::{pokemon_typing::PokemonTyping, poketype::PokeType},
        util::stat_utils,
    },
};

#[derive(Clone, Serialize, Deserialize)]
pub struct Pokemon {
    pub level: u8,

    pub max_hp: u16,
    pub hp: u16,
    pub attack: u16,
    pub spattack: u16,
    pub defense: u16,
    pub spdefense: u16,
    pub speed: u16,

    pub typing: PokemonTyping,

    pub moves: [MoveName; 4],

    pub ability: Ability,
}

impl Pokemon {
    pub fn new(
        base_pokemon: BasePokemon,
        level: u8,
        moves: Vec<MoveName>,
        ability: Ability,
        ivs: HashMap<StatEnum, u8>,
        evs: HashMap<StatEnum, u8>,
        nature: Nature,
    ) -> Self {
        if level == 0 || level > 100 {
            panic!("Disallowed level")
        }

        let num_moves = moves.len();
        let unique_moves_len = moves
            .clone()
            .into_iter()
            .collect::<HashSet<MoveName>>()
            .len();
        if unique_moves_len != num_moves {
            panic!("Contains duplicate moves")
        }

        let moves_arr = match moves.as_slice() {
            [move1] => [*move1, MoveName::Empty, MoveName::Empty, MoveName::Empty],
            [move1, move2] => [*move1, *move2, MoveName::Empty, MoveName::Empty],
            [move1, move2, move3] => [*move1, *move2, *move3, MoveName::Empty],
            [move1, move2, move3, move4] => [*move1, *move2, *move3, *move4],
            _ => panic!("Unexpected number of moves!"),
        };

        let (boosted_stat, neg_stat) = nature.get_changed_stats();

        let mut stat_map: EnumMap<StatEnum, u16> = EnumMap::default();

        for stat_enum in StatEnum::iter() {
            assert!(
                ivs.contains_key(&stat_enum) && ivs[&stat_enum] <= 31,
                "Invalid iv for {stat_enum}"
            );
            assert!(evs.contains_key(&stat_enum), "Invalid ev for {stat_enum}");

            let iv = ivs[&stat_enum];
            let ev = evs[&stat_enum];

            let base_stat: u16 = base_pokemon.stat_for_enum(&stat_enum);

            let stat = if stat_enum == StatEnum::HP {
                stat_utils::calculate_hp_stat(level, base_stat, iv, ev)
            } else {
                let nature_mult: f32 = if boosted_stat == neg_stat {
                    1.0
                } else if boosted_stat == stat_enum {
                    1.1
                } else if neg_stat == stat_enum {
                    0.9
                } else {
                    1.0
                };

                stat_utils::calculate_non_hp_stat(level, base_stat, iv, ev, nature_mult)
            };

            stat_map[stat_enum] = stat;
        }

        Self {
            level,
            max_hp: stat_map[StatEnum::HP],
            hp: stat_map[StatEnum::HP],
            attack: stat_map[StatEnum::Attack],
            spattack: stat_map[StatEnum::SpecialAttack],
            defense: stat_map[StatEnum::Defense],
            spdefense: stat_map[StatEnum::SpecialDefense],
            speed: stat_map[StatEnum::Speed],
            typing: base_pokemon.typing,
            moves: moves_arr,
            ability,
        }
    }

    pub fn get_move_for_action(&self, action: &Action) -> Option<MoveName> {
        match action {
            Action::Move(MoveSlot::Slot0) => Some(self.moves[0]),
            Action::Move(MoveSlot::Slot1) => Some(self.moves[1]),
            Action::Move(MoveSlot::Slot2) => Some(self.moves[2]),
            Action::Move(MoveSlot::Slot3) => Some(self.moves[3]),
            _ => None,
        }
    }

    pub fn get_stat_value(&self, stat: StatEnum) -> u16 {
        match stat {
            StatEnum::HP => self.hp,
            StatEnum::Attack => self.attack,
            StatEnum::Defense => self.defense,
            StatEnum::SpecialAttack => self.spattack,
            StatEnum::SpecialDefense => self.spdefense,
            StatEnum::Speed => self.speed,
        }
    }

    pub fn get_idx_for_move_name(&self, move_name: &MoveName) -> usize {
        self.moves
            .iter()
            .position(|target| move_name == target)
            .expect("Trying to find move_name that doesn't belong to this pokemon")
    }
}
