use crate::{battle::state::BattleState, common::{has_kind::HasKind, registry::Registry}, event::{event_handler::EventHandler, event_handler_effect::EventHandlerEffect, event_queue::EventQueue, event_type::{EventKind, Event}}};

pub struct EventBus {
    pub registry: Registry<Event, dyn EventHandler>,
    pub event_queue: EventQueue,
}

impl EventBus {
    pub fn new() -> Self {
        EventBus {
            registry: Registry::new(),
            event_queue: EventQueue::new(),
        }
    }

    pub fn publish(&mut self, event: &Event, battle_state: &mut BattleState) {
        if self.registry.contains(&event.kind()) {
            let len = self.registry.get(&event.kind()).len(); // temporary copy of length
            for i in 0..len {
                let effects = self.registry.get(&event.kind())[i].handle(event, battle_state);
                self.process_event_effects(effects, battle_state);
            }
        }
    }

    fn process_event_effects(&mut self, effects: Vec<EventHandlerEffect>, battle_state: &mut BattleState) {
        for effect in effects {
            self.process_event_effect(effect, battle_state);
        }
    }

    fn process_event_effect(&mut self, effect: EventHandlerEffect, battle_state: &mut BattleState) {
        match effect {
            EventHandlerEffect::Damage(_) => {panic!("Not implemented yet");},
        }
    }
}