use crate::{battle::state::BattleState, core::{pokemove::move_name::MoveName, poketype::{effectiveness, pokemon_typing::PokemonTyping, poketype::PokeType}, util::damage_utils}, dex::move_dex, query::{payload::PayloadQuery, query::Query, query_bus::QueryBus}};

pub struct BattleEngine;

impl BattleEngine {
    pub fn try_use_move(battle_state: &mut BattleState, query_bus: &QueryBus, is_trainer_1: bool, move_name: MoveName) {
        let can_execute = BattleEngine::check_move_execution(battle_state, query_bus, is_trainer_1, move_name);
        
        if can_execute {
            BattleEngine::single_hit_execution(battle_state, query_bus, is_trainer_1, move_name);
        }

    }

    fn single_hit_execution(battle_state: &mut BattleState, query_bus: &QueryBus, is_trainer_1: bool, move_name: MoveName) {
        let damage = BattleEngine::calculate_damage(battle_state, query_bus, is_trainer_1, move_name);
        BattleEngine::deal_damage(battle_state, query_bus,!is_trainer_1, damage);
    }

    pub fn deal_damage(battle_state: &mut BattleState, query_bus: &QueryBus, target_is_trainer_1: bool, damage: u32) {

        let final_damage_query = Query::FinalDamage(PayloadQuery::u32(target_is_trainer_1));
        query_bus.query(&final_damage_query, battle_state);

        let final_damage = final_damage_query.into_payload_query().get_u32();
        let caused_faint = battle_state.get_side_mut(target_is_trainer_1).take_damage(final_damage);

        if caused_faint {
            // Handle fainting logic
        }
    }

    fn calculate_damage(battle_state: &mut BattleState, query_bus: &QueryBus, is_trainer_1: bool, move_name: MoveName) -> u32 {
        let base_power_query = Query::OnBasePower(PayloadQuery::vec_f32(is_trainer_1));
        query_bus.query(&base_power_query, battle_state);
        let modified_base_power = base_power_query.into_payload_query().as_combined_modifier();

        let atk_query = Query::OnAtk(PayloadQuery::vec_f32(is_trainer_1));
        query_bus.query(&atk_query, battle_state);
        let modified_atk = atk_query.into_payload_query().as_combined_modifier();

        let def_query = Query::OnDef(PayloadQuery::vec_f32(is_trainer_1));
        query_bus.query(&def_query, battle_state);
        let modified_def = def_query.into_payload_query().as_combined_modifier();

        let mod1_query = Query::OnMod1(PayloadQuery::vec_f32(is_trainer_1));
        query_bus.query(&mod1_query, battle_state);
        let mod1 = mod1_query.into_payload_query().as_combined_modifier();

        let mod2_query = Query::OnMod2(PayloadQuery::vec_f32(is_trainer_1));
        query_bus.query(&mod2_query, battle_state);
        let mod2 = mod2_query.into_payload_query().as_combined_modifier();

        let mod3_query = Query::OnMod3(PayloadQuery::vec_f32(is_trainer_1));
        query_bus.query(&mod3_query, battle_state);
        let mod3 = mod3_query.into_payload_query().as_combined_modifier();

        let is_crit_query = Query::IsCrit(PayloadQuery::bool(is_trainer_1));
        query_bus.query(&is_crit_query, battle_state);
        let is_crit = is_crit_query.into_payload_query().get_bool();

        let crit_mult: f32 = if !is_crit {
            1.0
        } else {
            let crit_mult_query = Query::CritMult(PayloadQuery::f32(is_trainer_1)); 
            query_bus.query(&crit_mult_query, battle_state);      
            crit_mult_query.into_payload_query().get_f32()
        };

        let r = (100 - battle_state.get_rand_num(16));

        let stab_mult_query = Query::StabMult(PayloadQuery::f32_with_default(is_trainer_1, 1.0));
        query_bus.query(&stab_mult_query, battle_state);
        let stab_mult = stab_mult_query.into_payload_query().get_f32();

        let move_type = move_dex::get_move_data(&move_name).move_type;
        let (type1_mult, type2_mult) = match battle_state.get_active_pokemon(!is_trainer_1).typing {
            PokemonTyping::MonoType(t) => (
                effectiveness::type_effectiveness(move_type, t),
                1.0 as f32,
            ),
            PokemonTyping::DualType(t1, t2) => (
                effectiveness::type_effectiveness(move_type, t1),
                effectiveness::type_effectiveness(move_type, t2),
            )
        };

        damage_utils::get_damage_for_move(
            battle_state.get_active_pokemon(is_trainer_1).level,
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

    fn check_move_execution(battle_state: &mut BattleState, query_bus: &QueryBus, is_trainer_1: bool, move_name: MoveName) -> bool {
        let invuln_query = Query::CheckInvulnerability(PayloadQuery::bool_with_default(is_trainer_1, true));
        query_bus.query(&invuln_query, battle_state);
        if !invuln_query.into_payload_query().get_bool() {
            return false;
        }

        let immunity_query = Query::CheckImmunity(PayloadQuery::bool_with_default(is_trainer_1, true));
        query_bus.query(&immunity_query, battle_state);
        if !immunity_query.into_payload_query().get_bool() {
            return false;
        }

        let try_hit_query = Query::TryHit(PayloadQuery::bool_with_default(is_trainer_1, true));
        query_bus.query(&try_hit_query, battle_state);
        if !try_hit_query.into_payload_query().get_bool() {
            return false;
        }

        true
    }
}