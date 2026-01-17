use std::sync::Arc;

use super::actions::Action;
use crate::{
    battle::{
        actions::SwitchSlot,
        battle_context::BattleContext,
        battle_engine::BattleEngine,
        battle_input::{BattleInput, SingleInput},
        battle_request::{ActionResponse, BattleRequest, SingleBattleRequest},
        state::BattleState,
        static_battle_handler::StaticBattleHandler,
        turn_state::TurnState,
    },
    common::context::MoveContext,
    core::{
        pokemon::{pokemon::Pokemon, stat_enum::StatEnum},
        pokemove::{move_name::MoveName, move_target::MoveTarget},
        poketype::{effectiveness, poketype::PokeType},
        util::damage_utils,
    },
    dex::pokemove::move_dex,
    event::{
        event_bus::EventBus,
        event_queue::EventQueue,
        event_type::{BeforeMoveEvent, CanApplyMoveEvent, DamageEvent, Event, OnPriorityEvent},
    },
    query::{payload::PayloadMoveQuery, query::Query, query_bus::QueryBus},
};

pub struct Battle {
    battle_state: BattleState,
    event_bus: EventBus,
    query_bus: QueryBus,
}

impl Battle {
    pub fn new() -> Self {
        let mut event_bus = EventBus::new();
        let mut query_bus = QueryBus::new();

        // register defaults
        query_bus
            .registry
            .add_handler(Arc::new(StaticBattleHandler));

        Self {
            battle_state: BattleState::new(),
            event_bus,
            query_bus,
        }
    }

    pub fn start_battle(&mut self) {
        let first_trainer = self.resolve_action_order(
            &Action::Switch(SwitchSlot::Slot0),
            &Action::Switch(SwitchSlot::Slot0),
        );
        BattleEngine::switch_in_pokemon(&mut self.battle_context(), first_trainer, 0);
        BattleEngine::switch_in_pokemon(&mut self.battle_context(), !first_trainer, 0);
    }

    pub fn process_input(&mut self, input: BattleInput) -> BattleRequest {
        match input {
            (None, None) => panic!("Illegal input: both trainers provided no action"),
            (Some(SingleInput::SwitchInInput(slot1)), Some(SingleInput::SwitchInInput(slot2))) => {
                todo!("Handle double switch-in")
            }
            (Some(SingleInput::SwitchInInput(slot1)), None) => {
                todo!("Handle single switch-in for trainer 1")
            }
            (None, Some(SingleInput::SwitchInInput(slot2))) => {
                todo!("Handle single switch-in for trainer 2")
            }
            (
                Some(SingleInput::StandardInput(action1)),
                Some(SingleInput::StandardInput(action2)),
            ) => self.process_actions(action1, action2),
            _ => panic!("Illegal input combination"),
        }
    }

    fn process_actions(&mut self, action1: Action, action2: Action) -> BattleRequest {
        let mut turn_state = TurnState::new();
        self.event_bus.publish(
            &Event::BeginTurn,
            &mut self.battle_state,
            &mut self.query_bus,
            &mut turn_state,
        );
        let action1_first = self.resolve_action_order(&action1, &action2);

        if action1_first {
            self.process_action(true, &action1, &mut turn_state);
            // TODO: Handle mid-turn switch e.g. U-Turn
            if !self.battle_state.get_active_pokemon(false).is_fainted() {
                self.process_action(false, &action2, &mut turn_state);
            }
        } else {
            self.process_action(false, &action2, &mut turn_state);
            // TODO: Handle mid-turn switch e.g. U-Turn
            if !self.battle_state.get_active_pokemon(true).is_fainted() {
                self.process_action(true, &action1, &mut turn_state);
            }
        }

        BattleEngine::queue_after_turn_effects(&mut self.battle_context(), &mut turn_state);
        self.event_bus.drain_event_queue(
            &mut self.battle_state,
            &mut self.query_bus,
            &mut turn_state,
        );

        self.generate_battle_request_from_turn_state(&turn_state)
    }

    fn process_action(
        &mut self,
        is_trainer_1: bool,
        action: &Action,
        turn_state: &mut TurnState,
    ) -> ActionResponse {
        if action.is_switch() {
            BattleEngine::switch_pokemon(
                &mut self.battle_context(),
                is_trainer_1,
                action.get_switch_index(),
            );
        } else {
            let move_name = self
                .battle_state
                .get_move_for_move_action(is_trainer_1, &action);
            let move_context = Battle::create_move_context(move_name, is_trainer_1);

            BattleEngine::try_use_move(&mut self.battle_context(), &move_context, turn_state);
        }

        self.event_bus
            .drain_event_queue(&mut self.battle_state, &mut self.query_bus, turn_state);
        ActionResponse::Continue
    }

    // true is action1 goes first, false is action2 goes first
    fn resolve_action_order(&mut self, action1: &Action, action2: &Action) -> bool {
        let mut battle_context = self.battle_context();
        let priority1 = Self::get_action_priority(&mut battle_context, true, action1);
        let priority2 = Self::get_action_priority(&mut battle_context, false, action2);

        if priority1 > priority2 {
            true
        } else if priority2 > priority1 {
            false
        } else {
            BattleEngine::resolve_speed_order(&mut battle_context)
        }
    }

    fn get_action_priority(
        battle_context: &mut BattleContext,
        is_trainer_1: bool,
        action: &Action,
    ) -> i8 {
        let move1_option = battle_context
            .battle_state
            .get_move_for_action(is_trainer_1, action);

        Self::get_move_priority(battle_context, move1_option, action, is_trainer_1)
    }

    fn get_move_priority(
        battle_context: &mut BattleContext,
        optional_move: Option<MoveName>,
        action: &Action,
        is_trainer_1: bool,
    ) -> i8 {
        if action.is_switch() {
            6
        } else {
            let move_name = optional_move.expect("Expected a move for non-switch action");
            let context = Battle::create_move_context(move_name, is_trainer_1);
            let mut priority_query = Query::OnPriority(PayloadMoveQuery::i8(context));
            battle_context
                .query_bus
                .query(&mut priority_query, battle_context.battle_state);
            priority_query.into_payload_move_query().get_i8()
        }
    }

    fn create_move_context(move_name: MoveName, is_trainer_1: bool) -> MoveContext {
        let pokemove = move_dex::get_move_data(&move_name);
        MoveContext {
            src_trainer: is_trainer_1,
            target_trainer: match pokemove.target {
                MoveTarget::Opponent => !is_trainer_1,
                MoveTarget::User => is_trainer_1,
            },
            move_name,
            pokemove,
        }
    }

    fn generate_battle_request_from_turn_state(&self, turn_state: &TurnState) -> BattleRequest {
        match turn_state.fainted_sides.as_slice() {
            [] => BattleRequest::Request(
                Some(SingleBattleRequest::ActionRequest),
                Some(SingleBattleRequest::ActionRequest),
            ),
            [true] => {
                if self.battle_state.get_side(true).out_of_usable_pokemon() {
                    BattleRequest::BattleEnded(true)
                } else {
                    BattleRequest::Request(Some(SingleBattleRequest::SwitchInRequest), None)
                }
            }
            [false] => {
                if self.battle_state.get_side(false).out_of_usable_pokemon() {
                    BattleRequest::BattleEnded(false)
                } else {
                    BattleRequest::Request(None, Some(SingleBattleRequest::SwitchInRequest))
                }
            }
            [true, false] => {
                if self.battle_state.get_side(true).out_of_usable_pokemon() {
                    BattleRequest::BattleEnded(true)
                } else if self.battle_state.get_side(false).out_of_usable_pokemon() {
                    BattleRequest::BattleEnded(false)
                } else {
                    BattleRequest::Request(
                        Some(SingleBattleRequest::SwitchInRequest),
                        Some(SingleBattleRequest::SwitchInRequest),
                    )
                }
            }
            [false, true] => {
                if self.battle_state.get_side(false).out_of_usable_pokemon() {
                    BattleRequest::BattleEnded(false)
                } else if self.battle_state.get_side(true).out_of_usable_pokemon() {
                    BattleRequest::BattleEnded(true)
                } else {
                    BattleRequest::Request(
                        Some(SingleBattleRequest::SwitchInRequest),
                        Some(SingleBattleRequest::SwitchInRequest),
                    )
                }
            }
            _ => panic!("Unexpected turn state fainted sides"),
        }
    }

    fn battle_context(&mut self) -> BattleContext {
        BattleContext {
            battle_state: &mut self.battle_state,
            query_bus: &mut self.query_bus,
            event_queue: &mut self.event_bus.event_queue,
            event_registry: &mut self.event_bus.registry,
        }
    }
}
