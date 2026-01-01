use std::{collections::HashMap, rc::Rc, sync::{Arc, LazyLock}};

use enum_map::{enum_map, EnumMap};

use crate::{ability_map, battle::state::BattleState, core::{ability::ability::Ability, pokemon::pokemon::Pokemon, poketype::poketype::PokeType}, dex::combined_handler::CombinedHandler, event::event_handler::EventHandler, handler};

handler!{Blaze ( state ) {
    queries {
        OnBasePower ( payload ) => {
            let is_boosted = {
                let src_pokemon: &Pokemon = state.get_active_pokemon(payload.context.src_trainer);

                let move_type = payload.context.pokemove.move_type;
                move_type == PokeType::Fire && src_pokemon.hp <= src_pokemon.max_hp / 3
            };

            if is_boosted {
                payload;
            }
        },
    }
}}

handler!{Torrent ( state ) {
    queries {
        OnBasePower ( payload ) => {
            let is_boosted = {
                let src_pokemon: &Pokemon = state.get_active_pokemon(payload.context.src_trainer);

                let move_type = payload.context.pokemove.move_type;
                move_type == PokeType::Water && src_pokemon.hp <= src_pokemon.max_hp / 3
            };

            if is_boosted {
                payload;
            }

        },
    }
}}

handler!{Overgrow ( state ) {
    queries {
        OnBasePower ( payload ) => {
            let is_boosted = {
                let src_pokemon: &Pokemon = state.get_active_pokemon(payload.context.src_trainer);

                let move_type = payload.context.pokemove.move_type;
                move_type == PokeType::Grass && src_pokemon.hp <= src_pokemon.max_hp / 3
            };

            if is_boosted {
                payload;
            }

        },
    }
}}

static ABILITY_MAP: LazyLock<EnumMap<Ability, Arc<dyn CombinedHandler>>> = ability_map!{ABILITY_MAP {
    Blaze,
    Torrent,
    Overgrow,
}};

pub fn get_ability_handler(ability: &Ability) -> &Arc<dyn CombinedHandler> {
    &ABILITY_MAP[*ability]
}