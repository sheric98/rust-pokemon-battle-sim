use crate::{
    battle::state::BattleState,
    common::registry::Registry,
    event::{event_handler::EventHandler, event_queue::EventQueue, event_type::Event},
    query::{query::Query, query_bus::QueryBus, query_handler::QueryHandler},
};

pub struct BattleContext<'a> {
    pub battle_state: &'a mut BattleState,
    pub query_bus: &'a mut QueryBus,
    pub event_queue: &'a mut EventQueue,
    pub event_registry: &'a mut Registry<Event, dyn EventHandler>,
}
