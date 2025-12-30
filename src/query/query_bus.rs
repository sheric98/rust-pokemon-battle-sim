use crate::{battle::state::BattleState, common::{has_kind::HasKind, registry::Registry}, event::{event_handler::EventHandler, event_handler_effect::EventHandlerEffect, event_queue::EventQueue, event_type::{EventKind, EventType}}, query::{query::Query, query_handler::QueryHandler}};

pub struct QueryBus {
    pub registry: Registry<Query, dyn QueryHandler>,
}

impl QueryBus {
    pub fn new() -> Self {
        QueryBus {
            registry: Registry::new(),
        }
    }

    pub fn query(&self, query: &Query, battle_state: &mut BattleState) {
        if self.registry.contains(&query.kind()) {
            let len = self.registry.get(&query.kind()).len(); // temporary copy of length
            for i in 0..len {
                self.registry.get(&query.kind())[i].handle(query, battle_state);
            }
        }
    }
}