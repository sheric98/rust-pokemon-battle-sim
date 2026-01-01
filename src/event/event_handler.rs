use crate::battle::state::BattleState;
use crate::common::subscriber::Subscriber;
use crate::event::event_handler_effect::EventHandlerEffect;
use crate::event::event_type::Event;

pub trait EventHandler: Subscriber<Event> {
    fn handle(&self, event: &Event, battle_state: &BattleState) -> Vec<EventHandlerEffect>;
}
