use std::{collections::HashMap, sync::Arc};

use enum_map::EnumMap;
use serde::{Deserialize, Serialize};

use crate::{
    core::{
        pokemon::{pokemon::Pokemon, stat_enum::StatEnum},
        status::{status::Status, volatile_status::VolatileStatus},
    },
    dex::{ability::ability_handlers, combined_handler::CombinedHandler, status::status_handlers},
    event::event_handler::EventHandler,
    query::query_handler::QueryHandler,
};

#[derive(Clone, Serialize)]
pub struct PokemonBattleInstance {
    pub pokemon: Pokemon,
    pub status: Option<Status>,
    #[serde(skip)]
    pub volatile_statuses: HashMap<VolatileStatus, Arc<dyn CombinedHandler>>,
    pub trainer_side: bool,
    pub sleep_turns: u8,
    pub badly_poison_turns: u8,
    #[serde(skip)]
    pub boosts: EnumMap<StatEnum, i8>,

    #[serde(skip)]
    pub ability_handler: Arc<dyn CombinedHandler>,
    #[serde(skip)]
    pub status_handler: Option<Arc<dyn CombinedHandler>>,
}

impl PokemonBattleInstance {
    pub fn new(pokemon: Pokemon, trainer_side: bool) -> Self {
        let ability = pokemon.ability;

        Self {
            pokemon,
            trainer_side,
            status: None,
            sleep_turns: 0,
            badly_poison_turns: 0,
            volatile_statuses: HashMap::new(),
            boosts: EnumMap::default(),

            ability_handler: ability_handlers::get_ability_handler(&ability, trainer_side),
            status_handler: None,
        }
    }

    pub fn set_status(&mut self, status: Status) {
        self.status = Some(status);
        self.status_handler = Some(status_handlers::get_status_handler(
            status,
            self.trainer_side,
        ));
    }

    pub fn get_volatile_status_handler(
        &self,
        status: &VolatileStatus,
    ) -> &Arc<dyn CombinedHandler> {
        self.volatile_statuses
            .get(status)
            .expect("Trying to get status handler that does not exist")
    }

    pub fn remove_volatile_status(&mut self, status: &VolatileStatus) {
        self.volatile_statuses.remove(status);
    }

    pub fn clear_status(&mut self) {
        self.status = None;
        self.status_handler = None;
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
        self.badly_poison_turns = 0;

        self.boosts.clear();
    }

    pub fn get_all_event_handlers(&self) -> Vec<Arc<dyn EventHandler>> {
        let mut handlers: Vec<Arc<dyn EventHandler>> = vec![];

        handlers.push(self.ability_handler.clone());

        if let Some(status_handler) = &self.status_handler {
            handlers.push(status_handler.clone());
        }

        for volatile_status_handler in self.volatile_statuses.values() {
            handlers.push(volatile_status_handler.clone());
        }

        handlers
    }

    pub fn get_all_query_handlers(&self) -> Vec<Arc<dyn QueryHandler>> {
        let mut handlers: Vec<Arc<dyn QueryHandler>> = vec![];

        handlers.push(self.ability_handler.clone());

        if let Some(status_handler) = &self.status_handler {
            handlers.push(status_handler.clone());
        }

        for volatile_status_handler in self.volatile_statuses.values() {
            handlers.push(volatile_status_handler.clone());
        }

        handlers
    }
}
