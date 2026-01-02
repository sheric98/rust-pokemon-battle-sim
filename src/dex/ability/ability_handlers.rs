use std::{
    collections::HashMap,
    rc::Rc,
    sync::{Arc, LazyLock},
};

use enum_map::{EnumMap, enum_map};

use crate::{
    ability_map,
    battle::state::BattleState,
    core::{ability::ability::Ability, pokemon::pokemon::Pokemon, poketype::poketype::PokeType},
    dex::combined_handler::CombinedHandler,
    event::event_handler::EventHandler,
    handler,
};

handler! {Blaze ( s, state ) {
    queries {
        OnBasePower ( payload ) => {
            if payload.context.src_trainer != s.trainer_side {
                return
            }

            let is_boosted = {
                let src_pokemon: &Pokemon = &state.get_active_pokemon(payload.context.src_trainer).pokemon;

                let move_type = payload.context.pokemove.move_type;
                move_type == PokeType::Fire && src_pokemon.hp <= src_pokemon.max_hp / 3
            };

            if is_boosted {
                payload.get_vec_f32().push(1.5);
            }
        },
    }
}}

handler! {Torrent ( s, state ) {
    queries {
        OnBasePower ( payload ) => {
            if payload.context.src_trainer != s.trainer_side {
                return
            }

            let is_boosted = {
                let src_pokemon: &Pokemon = &state.get_active_pokemon(payload.context.src_trainer).pokemon;

                let move_type = payload.context.pokemove.move_type;
                move_type == PokeType::Water && src_pokemon.hp <= src_pokemon.max_hp / 3
            };

            if is_boosted {
                payload.get_vec_f32().push(1.5);
            }

        },
    }
}}

handler! {Overgrow ( s, state ) {
    queries {
        OnBasePower ( payload ) => {
            if payload.context.src_trainer != s.trainer_side {
                return
            }

            let is_boosted = {
                let src_pokemon: &Pokemon = &state.get_active_pokemon(payload.context.src_trainer).pokemon;

                let move_type = payload.context.pokemove.move_type;
                move_type == PokeType::Grass && src_pokemon.hp <= src_pokemon.max_hp / 3
            };

            if is_boosted {
                payload.get_vec_f32().push(1.5);
            }

        },
    }
}}

pub fn get_ability_handler(ability: &Ability, trainer_side: bool) -> Arc<dyn CombinedHandler> {
    match ability {
        Ability::Blaze => Arc::new(Blaze::new(trainer_side)) as Arc<dyn CombinedHandler>,
        Ability::Overgrow => Arc::new(Overgrow::new(trainer_side)) as Arc<dyn CombinedHandler>,
        Ability::Torrent => Arc::new(Torrent::new(trainer_side)) as Arc<dyn CombinedHandler>,
    }
}
