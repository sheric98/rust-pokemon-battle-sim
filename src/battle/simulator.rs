use std::sync::Arc;

use super::actions::Action;
use crate::{
    battle::{
        battle_engine::BattleEngine, state::BattleState, static_battle_handler::StaticBattleHandler,
    },
    core::{
        pokemon::pokemon::Pokemon,
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
    query::{
        payload::{MoveQueryContext, PayloadMoveQuery},
        query::Query,
        query_bus::QueryBus,
    },
};

pub struct BattleSimulator {
    battle_state: BattleState,
    event_bus: EventBus,
    query_bus: QueryBus,
}

impl BattleSimulator {
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

    pub fn process_actions(&mut self, action1: Action, action2: Action) {
        self.event_bus
            .publish(&Event::BeginTurn, &mut self.battle_state);
        let action1_first = self.resolve_action_order(&action1, &action2);

        if action1_first {
            self.process_action(true, &action1);
            self.process_action(false, &action2);
        } else {
            self.process_action(false, &action2);
            self.process_action(true, &action1);
        }
    }

    fn process_action(&mut self, is_trainer_1: bool, action: &Action) {
        if action.is_switch() {
            // Handle switch action
            // This is a placeholder for actual switch logic
            return;
        } else {
            let move_name = self
                .battle_state
                .get_move_for_move_action(is_trainer_1, &action);
            let move_context = BattleSimulator::create_move_context(move_name, is_trainer_1);

            BattleEngine::try_use_move(&mut self.battle_state, &self.query_bus, &move_context);
        }
    }

    // true is action1 goes first, false is action2 goes first
    fn resolve_action_order(&mut self, action1: &Action, action2: &Action) -> bool {
        let priority1 = self.get_action_priority(true, action1);
        let priority2 = self.get_action_priority(false, action2);

        if priority1 > priority2 {
            true
        } else if priority2 > priority1 {
            false
        } else {
            let speed1 = self.battle_state.get_active_pokemon(true).speed;
            let speed2 = self.battle_state.get_active_pokemon(false).speed;
            if speed1 > speed2 {
                true
            } else if speed2 > speed1 {
                false
            } else {
                self.battle_state.get_random_check(1, 2)
            }
        }
    }

    fn get_action_priority(&mut self, is_trainer_1: bool, action: &Action) -> i8 {
        let move1_option = self.battle_state.get_move_for_action(is_trainer_1, action);

        self.get_move_priority(move1_option, action, is_trainer_1)
    }

    fn get_move_priority(
        &mut self,
        optional_move: Option<MoveName>,
        action: &Action,
        is_trainer_1: bool,
    ) -> i8 {
        if action.is_switch() {
            6
        } else {
            let move_name = optional_move.expect("Expected a move for non-switch action");
            let context = BattleSimulator::create_move_context(move_name, is_trainer_1);
            let mut priority_query = Query::OnPriority(PayloadMoveQuery::i8(context));
            self.query_bus
                .query(&mut priority_query, &mut self.battle_state);
            priority_query.into_payload_move_query().get_i8()
        }
    }

    fn create_move_context(move_name: MoveName, is_trainer_1: bool) -> MoveQueryContext {
        let pokemove = move_dex::get_move_data(&move_name);
        MoveQueryContext {
            src_trainer: is_trainer_1,
            target_trainer: match pokemove.target {
                MoveTarget::Opponent => !is_trainer_1,
                MoveTarget::User => is_trainer_1,
            },
            move_name,
            pokemove,
        }
    }
}
