use super::actions::Action;
use crate::{battle::state::BattleState, core::{pokemon::pokemon::Pokemon, pokemove::move_name::MoveName, poketype::{effectiveness, poketype::PokeType}, util::damage_utils}, dex::move_dex, event::{event_bus::EventBus, event_queue::EventQueue, event_type::{BeforeMoveEvent, CanApplyMoveEvent, DamageEvent, EventType, OnPriorityEvent}, payload::PayloadEvent}};


pub struct BattleSimulator {
    battle_state: BattleState,
    event_bus: EventBus,
}

impl BattleSimulator {
    pub fn new() -> Self {
        Self {
            battle_state: BattleState::new(),
            event_bus: EventBus::new(),
        }
    }

    pub fn process_actions(&mut self, action1: Action, action2: Action)
    {
        self.event_bus.publish(&EventType::BeginTurn, &mut self.battle_state);
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
            let move_name = self.battle_state.get_move_for_move_action(is_trainer_1, &action);
            let before_move_event = EventType::BeforeMove(BeforeMoveEvent { move_name, should_cancel: false });

            self.event_bus.publish(&before_move_event, &mut self.battle_state);
            if before_move_event.into_before_move().should_cancel {
                return;
            }

            // Check if move can be executed
            if !self.check_move_execution(move_name) {
                return;
            }

            self.single_hit_execution(is_trainer_1, move_name);
        }
    }

    fn single_hit_execution(&mut self, is_trainer_1: bool, move_name: MoveName) {
        let damage = self.calculate_damage(is_trainer_1, move_name);
        self.deal_damage(!is_trainer_1, damage);
    }

    fn deal_damage(&mut self, is_trainer_1: bool, damage: u32) {
        let damage_event = EventType::Damage(DamageEvent {
            target_trainer_1_side: !is_trainer_1,
            damage,
            caused_faint: false,
        });

        self.event_bus.publish(&damage_event, &mut self.battle_state);
        let damage_event_result = damage_event.into_damage_event();

        let final_damage = damage_event_result.damage;
        let caused_faint = self.battle_state.get_side_mut(!is_trainer_1).take_damage(final_damage);

        if caused_faint {
            // Handle fainting logic
        }
    }

    fn calculate_damage(&mut self, is_trainer_1: bool, move_name: MoveName) -> u32 {
        let base_power_event = EventType::OnBasePower(PayloadEvent::vec_f32(is_trainer_1));
        self.event_bus.publish(&base_power_event, &mut self.battle_state);
        let modified_base_power = base_power_event.into_payload_event().as_combined_modifier();

        let atk_event = EventType::OnAtk(PayloadEvent::vec_f32(is_trainer_1));
        self.event_bus.publish(&atk_event, &mut self.battle_state);
        let modified_atk = atk_event.into_payload_event().as_combined_modifier();

        let def_event = EventType::OnDef(PayloadEvent::vec_f32(is_trainer_1));
        self.event_bus.publish(&def_event, &mut self.battle_state);
        let modified_def = def_event.into_payload_event().as_combined_modifier();

        let mod1_event = EventType::OnMod1(PayloadEvent::vec_f32(is_trainer_1));
        self.event_bus.publish(&mod1_event, &mut self.battle_state);
        let mod1 = mod1_event.into_payload_event().as_combined_modifier();

        let mod2_event = EventType::OnMod2(PayloadEvent::vec_f32(is_trainer_1));
        self.event_bus.publish(&mod2_event, &mut self.battle_state);
        let mod2 = mod2_event.into_payload_event().as_combined_modifier();

        let mod3_event = EventType::OnMod3(PayloadEvent::vec_f32(is_trainer_1));
        self.event_bus.publish(&mod3_event, &mut self.battle_state);
        let mod3 = mod3_event.into_payload_event().as_combined_modifier();

        let is_crit_event = EventType::IsCrit(PayloadEvent::bool(is_trainer_1));
        self.event_bus.publish(&is_crit_event, &mut self.battle_state);
        let is_crit = is_crit_event.into_payload_event().get_bool();

        let crit_mult: f32 = if !is_crit {
            1.0
        } else {
            let crit_mult_event = EventType::CritMult(PayloadEvent::f32(is_trainer_1)); 
            self.event_bus.publish(&crit_mult_event, &mut self.battle_state);      
            crit_mult_event.into_payload_event().get_f32()
        };

        let r = (100 - self.battle_state.get_rand_num(16));

        let stab_mult_event = EventType::StabMult(PayloadEvent::f32_with_default(is_trainer_1, 1.0));
        self.event_bus.publish(&stab_mult_event, &mut self.battle_state);
        let stab_mult = stab_mult_event.into_payload_event().get_f32();

        let type1_mult = self.get_type_effectiveness(
            move_dex::get_move_data(&move_name).move_type,
            self.battle_state.get_active_pokemon(!is_trainer_1).type1,
        );
        let type2_mult = self.get_type_effectiveness(
            move_dex::get_move_data(&move_name).move_type,
            self.battle_state.get_active_pokemon(!is_trainer_1).type2,
        );

        damage_utils::get_damage_for_move(
            self.battle_state.get_active_pokemon(is_trainer_1).level,
            modified_base_power,
            modified_atk,
            modified_def,
            mod1,
            mod2,
            mod3,
            crit_mult,
            r,
            stab_mult,
            type1_mult,
            type2_mult,
        )
    }

    fn get_type_effectiveness(&self, move_type: PokeType, defend_type: Option<PokeType>) -> f32 {
        if let Some(def_type) = defend_type {
            effectiveness::type_effectiveness(move_type, def_type)
        } else {
            1.0
        }
    }

    fn check_move_execution(&mut self, move_name: MoveName) -> bool {
        let invuln_event = EventType::CheckInvulnerability(CanApplyMoveEvent { move_name, can_apply: true });
        self.event_bus.publish(&invuln_event, &mut self.battle_state);
        if !invuln_event.into_can_apply_move().can_apply {
            return false;
        }

        let immunity_event = EventType::CheckImmunity(CanApplyMoveEvent { move_name, can_apply: true });
        self.event_bus.publish(&immunity_event, &mut self.battle_state);
        if !immunity_event.into_can_apply_move().can_apply {
            return false;
        }

        let try_hit_event = EventType::TryHit(CanApplyMoveEvent { move_name, can_apply: true });
        self.event_bus.publish(&try_hit_event, &mut self.battle_state);
        if !try_hit_event.into_can_apply_move().can_apply {
            return false;
        }

        true
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

        self.get_move_priority(move1_option, action)
    }

    fn get_move_priority(&mut self, optional_move: Option<MoveName>, action: &Action) -> i8 {
        if action.is_switch() {
            6
        } else {
            let pokemove = optional_move.expect("Expected a move for non-switch action");
            let priority_event = EventType::OnPriority(OnPriorityEvent {
                priority: 0,
                move_name: pokemove,
            });
            self.event_bus.publish(&priority_event, &mut self.battle_state);
            priority_event.into_on_priority().priority
        }
    }
}
