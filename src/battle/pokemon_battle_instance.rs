use std::sync::Arc;

use enum_map::EnumMap;
use serde::{Deserialize, Serialize};

use crate::{
    core::{
        pokemon::{pokemon::Pokemon, stat_enum::StatEnum},
        status::volatile_status::VolatileStatus,
    },
    dex::{ability::ability_handlers, combined_handler::CombinedHandler},
};

#[derive(Clone, Serialize)]
pub struct PokemonBattleInstance {
    pub pokemon: Pokemon,
    pub volatile_statuses: Vec<VolatileStatus>,
    #[serde(skip)]
    pub boosts: EnumMap<StatEnum, i8>,

    #[serde(skip)]
    pub ability_handler: Arc<dyn CombinedHandler>,
    #[serde(skip)]
    pub status_handler: Option<Arc<dyn CombinedHandler>>,

    #[serde(skip)]
    pub volatile_status_handlers: Vec<Arc<dyn CombinedHandler>>,
}

impl PokemonBattleInstance {
    pub fn new(pokemon: Pokemon, trainer_side: bool) -> Self {
        let ability = pokemon.ability;

        Self {
            pokemon,
            volatile_statuses: vec![],
            boosts: EnumMap::default(),

            ability_handler: ability_handlers::get_ability_handler(&ability, trainer_side),
            status_handler: None,

            volatile_status_handlers: vec![],
        }
    }

    pub fn reset(&mut self) {
        self.volatile_statuses.clear();
        self.volatile_status_handlers.clear();

        self.boosts.clear();
    }
}
