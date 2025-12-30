use crate::battle::state::BattleState;
use crate::event::event_handler_effect::EventHandlerEffect;
use crate::event::event_type::{EventKind, EventType};

pub trait EventHandler {
    fn subscriptions(&self) -> &'static [EventKind];
    fn handle(&self, event_type: &EventType, battle_state: &BattleState) -> Vec<EventHandlerEffect>;
    fn priority(&self) -> i32;
}
