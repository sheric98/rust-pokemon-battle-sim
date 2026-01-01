use crate::{battle::state::BattleState, core::{pokemove::move_name::MoveName, poketype::{effectiveness, pokemon_typing::PokemonTyping, poketype::PokeType}, util::damage_utils}, dex::pokemove::move_dex, query::{payload::{MoveQueryContext, PayloadMoveQuery}, query::Query, query_bus::QueryBus}};

pub struct BattleEngine;

impl BattleEngine {
    pub fn try_use_move(battle_state: &mut BattleState, query_bus: &QueryBus, move_context: &MoveQueryContext) {        
        let can_execute = BattleEngine::check_move_execution(battle_state, query_bus, move_context);
        
        if can_execute {
            BattleEngine::single_hit_execution(battle_state, query_bus, move_context);
        }

    }

    fn single_hit_execution(battle_state: &mut BattleState, query_bus: &QueryBus, move_context: &MoveQueryContext) {
        let damage = BattleEngine::calculate_damage(battle_state, query_bus, move_context);
        BattleEngine::deal_damage(battle_state, query_bus, move_context, damage);
    }

    pub fn deal_damage(battle_state: &mut BattleState, query_bus: &QueryBus, move_context: &MoveQueryContext, damage: u32) {

        let mut final_damage_query = Query::FinalDamage(PayloadMoveQuery::u32(*move_context));
        query_bus.query(&mut final_damage_query, battle_state);

        let final_damage = final_damage_query.into_payload_move_query().get_u32();
        let caused_faint = battle_state.get_side_mut(move_context.target_trainer).take_damage(final_damage);

        if caused_faint {
            // Handle fainting logic
        }
    }

    fn calculate_damage(battle_state: &mut BattleState, query_bus: &QueryBus, move_context: &MoveQueryContext) -> u32 {
        let mut base_power_query = Query::OnBasePower(PayloadMoveQuery::vec_f32(*move_context));
        query_bus.query(&mut base_power_query, battle_state);
        let modified_base_power = base_power_query.into_payload_move_query().as_combined_modifier();

        let mut atk_query = Query::OnAtk(PayloadMoveQuery::vec_f32(*move_context));
        query_bus.query(&mut atk_query, battle_state);
        let modified_atk = atk_query.into_payload_move_query().as_combined_modifier();

        let mut def_query = Query::OnDef(PayloadMoveQuery::vec_f32(*move_context));
        query_bus.query(&mut def_query, battle_state);
        let modified_def = def_query.into_payload_move_query().as_combined_modifier();

        let mut mod1_query = Query::OnMod1(PayloadMoveQuery::vec_f32(*move_context));
        query_bus.query(&mut mod1_query, battle_state);
        let mod1 = mod1_query.into_payload_move_query().as_combined_modifier();

        let mut mod2_query = Query::OnMod2(PayloadMoveQuery::vec_f32(*move_context));
        query_bus.query(&mut mod2_query, battle_state);
        let mod2 = mod2_query.into_payload_move_query().as_combined_modifier();

        let mut mod3_query = Query::OnMod3(PayloadMoveQuery::vec_f32(*move_context));
        query_bus.query(&mut mod3_query, battle_state);
        let mod3 = mod3_query.into_payload_move_query().as_combined_modifier();

        let mut is_crit_query = Query::IsCrit(PayloadMoveQuery::bool(*move_context));
        query_bus.query(&mut is_crit_query, battle_state);
        let is_crit = is_crit_query.into_payload_move_query().get_bool();

        let crit_mult: f32 = if !is_crit {
            1.0
        } else {
            let mut crit_mult_query = Query::CritMult(PayloadMoveQuery::f32(*move_context)); 
            query_bus.query(&mut crit_mult_query, battle_state);      
            crit_mult_query.into_payload_move_query().get_f32()
        };

        let r = (100 - battle_state.get_rand_num(16));

        let mut stab_mult_query = Query::StabMult(PayloadMoveQuery::f32_with_default(*move_context, 1.0));
        query_bus.query(&mut stab_mult_query, battle_state);
        let stab_mult = stab_mult_query.into_payload_move_query().get_f32();

        let move_type = move_dex::get_move_data(&move_context.move_name).move_type;
        let (type1_mult, type2_mult) = match battle_state.get_active_pokemon(move_context.target_trainer).typing {
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
            battle_state.get_active_pokemon(move_context.src_trainer).level,
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

    fn check_move_execution(battle_state: &mut BattleState, query_bus: &QueryBus, move_context: &MoveQueryContext) -> bool {
        let mut invuln_query = Query::CheckInvulnerability(PayloadMoveQuery::bool_with_default(*move_context, true));
        query_bus.query(&mut invuln_query, battle_state);
        if !invuln_query.into_payload_move_query().get_bool() {
            return false;
        }

        let mut immunity_query = Query::CheckImmunity(PayloadMoveQuery::bool_with_default(*move_context, true));
        query_bus.query(&mut immunity_query, battle_state);
        if !immunity_query.into_payload_move_query().get_bool() {
            return false;
        }

        let mut try_hit_query = Query::TryHit(PayloadMoveQuery::bool_with_default(*move_context, true));
        query_bus.query(&mut try_hit_query, battle_state);
        if !try_hit_query.into_payload_move_query().get_bool() {
            return false;
        }

        true
    }
}