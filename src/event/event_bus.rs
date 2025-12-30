use std::{collections::HashMap, rc::Rc};

use crate::{battle::state::BattleState, event::{event_handler::EventHandler, event_handler_effect::EventHandlerEffect, event_queue::EventQueue, event_type::{EventKind, EventType}}};

pub struct EventBus {
    subscribers: HashMap<EventKind, Vec<Rc<dyn EventHandler>>>,
    event_queue: EventQueue,
}

impl EventBus {
    pub fn new() -> Self {
        EventBus {
            subscribers: HashMap::new(),
            event_queue: EventQueue::new(),
        }
    }

    pub fn publish(&mut self, event: &EventType, battle_state: &mut BattleState) {
        if self.subscribers.contains_key(&event.kind()) {
            let len = self.subscribers[&event.kind()].len(); // temporary copy of length
            for i in 0..len {
                let effects = self.subscribers[&event.kind()][i].handle(event, battle_state);
                self.process_event_effects(effects, battle_state);
            }
        }
    }

    pub fn add_handlers(&mut self, handlers: Vec<Rc<dyn EventHandler>>) {
        for handler in handlers {
            self.add_handler(handler);
        }
    }

    fn add_handler(&mut self, handler: Rc<dyn EventHandler>) {
        handler.as_ref().subscriptions().iter().for_each(|event_kind| {
            let vec = self.subscribers.entry(*event_kind)
                .or_insert_with(Vec::new);

            let idx = vec
                .binary_search_by(|s| s.priority().cmp(&handler.priority()).reverse())
                .unwrap_or_else(|i| i);
            vec.insert(idx, handler.clone());
        } );
    }

    // TODO: Evaluate if this is the best way to remove handlers
    pub fn remove_handlers(&mut self, handlers: Vec<Rc<dyn EventHandler>>) {
        for handler in handlers {
            handler.as_ref().subscriptions().iter().for_each(|event_kind| {
                if let Some(subscribers) = self.subscribers.get_mut(event_kind) {
                    subscribers.retain(|s| !Rc::ptr_eq(s, &handler));
                }
            });
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