use crate::battle::state::BattleState;
use crate::common::subscriber::Subscriber;
use crate::event::event_handler_effect::EventHandlerEffect;
use crate::event::event_type::EventType;

pub trait EventHandler: Subscriber<EventType> {
    fn handle(&self, event_type: &EventType, battle_state: &BattleState) -> Vec<EventHandlerEffect>;
}
