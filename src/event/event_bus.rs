use crate::{
    battle::{
        battle_context::BattleContext,
        battle_engine::BattleEngine,
        state::BattleState,
        turn_state::{self, TurnState},
    },
    common::{has_kind::HasKind, registry::Registry},
    event::{
        event_handler::EventHandler,
        event_handler_effect::EventHandlerEffect,
        event_queue::EventQueue,
        event_type::{Event, EventKind},
    },
    query::{self, query_bus::QueryBus},
};

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

    pub fn publish(
        &mut self,
        event: &Event,
        battle_state: &mut BattleState,
        query_bus: &mut QueryBus,
        turn_state: &mut TurnState,
    ) {
        if self.registry.contains(&event.kind()) {
            let len = self.registry.get(&event.kind()).len(); // temporary copy of length
            for i in 0..len {
                let effects = self.registry.get(&event.kind())[i].handle(event, battle_state);
                self.process_event_effects(effects, battle_state, query_bus, turn_state);
            }
        }
    }

    pub fn drain_event_queue(
        &mut self,
        battle_state: &mut BattleState,
        query_bus: &mut QueryBus,
        turn_state: &mut TurnState,
    ) {
        while let Some(event) = self.event_queue.dequeue() {
            self.publish(&event, battle_state, query_bus, turn_state);
        }
    }

    fn process_event_effects(
        &mut self,
        effects: Vec<EventHandlerEffect>,
        battle_state: &mut BattleState,
        query_bus: &mut QueryBus,
        turn_state: &mut TurnState,
    ) {
        for effect in effects {
            self.process_event_effect(effect, battle_state, query_bus, turn_state);
        }
    }

    fn process_event_effect(
        &mut self,
        effect: EventHandlerEffect,
        battle_state: &mut BattleState,
        query_bus: &mut QueryBus,
        turn_state: &mut TurnState,
    ) {
        match effect {
            EventHandlerEffect::Damage(damage, target_trainer) => {
                BattleEngine::deal_damage(
                    &mut self.battle_context(battle_state, query_bus),
                    None,
                    Some(target_trainer),
                    turn_state,
                    damage,
                );
            }
            EventHandlerEffect::DamageAndHeal(damage, damage_target, heal_target) => {
                BattleEngine::deal_damage_and_heal(
                    &mut self.battle_context(battle_state, query_bus),
                    damage_target,
                    heal_target,
                    damage,
                    turn_state,
                );
            }
        }
    }

    fn battle_context<'a>(
        &'a mut self,
        battle_state: &'a mut BattleState,
        query_bus: &'a mut QueryBus,
    ) -> BattleContext<'a> {
        BattleContext {
            battle_state,
            query_bus,
            event_queue: &mut self.event_queue,
            event_registry: &mut self.registry,
        }
    }
}
