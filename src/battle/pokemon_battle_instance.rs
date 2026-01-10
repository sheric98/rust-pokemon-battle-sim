use std::sync::Arc;

use enum_map::EnumMap;
use serde::{Deserialize, Serialize};

use crate::{
    core::{
        pokemon::{pokemon::Pokemon, stat_enum::StatEnum},
        status::{status::Status, volatile_status::VolatileStatus},
    },
    dex::{ability::ability_handlers, combined_handler::CombinedHandler},
};

#[derive(Clone, Serialize)]
pub struct PokemonBattleInstance {
    pub pokemon: Pokemon,
    pub status: Option<Status>,
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
            status: None,
            volatile_statuses: vec![],
            boosts: EnumMap::default(),

            ability_handler: ability_handlers::get_ability_handler(&ability, trainer_side),
            status_handler: None,

            volatile_status_handlers: vec![],
        }
    }

    pub fn set_fainted(&mut self) {
        self.status = Some(Status::Faint);
    }

    pub fn is_fainted(&self) -> bool {
        match self.status {
            Some(Status::Faint) => true,
            _ => false,
        }
    }

    pub fn reset(&mut self) {
        self.volatile_statuses.clear();
        self.volatile_status_handlers.clear();

        self.boosts.clear();
    }
}
