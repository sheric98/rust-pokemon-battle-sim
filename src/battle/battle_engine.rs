use std::{cmp, sync::Arc};

use crate::{
    battle::{
        battle_context::BattleContext,
        pokemon_battle_instance::{self, PokemonBattleInstance},
        state::BattleState,
        turn_state::TurnState,
    },
    common::{context::MoveContext, registry::Registry},
    core::{
        pokemon::{self, boostable_stat::BoostableStat, stat_enum::StatEnum},
        pokemove::{
            move_category::MoveCategory, move_name::MoveName, secondary_effect::SecondaryEffect,
        },
        poketype::{effectiveness, pokemon_typing::PokemonTyping, poketype::PokeType},
        status::{status::Status, volatile_status::VolatileStatus},
        util::damage_utils,
    },
    dex::{combined_handler::CombinedHandler, pokemove::move_dex},
    event::{
        self,
        event_handler::EventHandler,
        event_queue::{self, EventQueue},
        event_type::{Event, FaintEvent},
    },
    query::{
        payload::PayloadMoveQuery,
        query::{
            CanApplyStatusQuery, CanApplyVolatileStatusQuery, FinalDamageQuery, MultiHitHitsQuery,
            MultiHitRangeQuery, OnStatQuery, Query, TryUseMoveQuery,
        },
        query_bus::QueryBus,
        query_handler::QueryHandler,
    },
};

pub struct BattleEngine;

impl BattleEngine {
    pub fn switch_pokemon(battle_context: &mut BattleContext, trainer: bool, switch_idx: usize) {
        BattleEngine::switch_out_pokemon(battle_context, trainer, switch_idx);
        BattleEngine::switch_in_pokemon(battle_context, trainer, switch_idx);
    }

    pub fn switch_in_pokemon(battle_context: &mut BattleContext, trainer: bool, switch_idx: usize) {
        let pokemon_battle_instance = battle_context
            .battle_state
            .get_side_mut(trainer)
            .set_active_pokemon(switch_idx);

        BattleEngine::register_handlers_for_pokemon(
            &mut battle_context.event_registry,
            &mut battle_context.query_bus.registry,
            pokemon_battle_instance,
        );
    }

    pub fn try_use_move(
        battle_context: &mut BattleContext,
        move_context: &MoveContext,
        turn_state: &mut TurnState,
    ) {
        let can_execute = BattleEngine::check_move_execution(battle_context, move_context);

        if !can_execute {
            return;
        }

        if !BattleEngine::check_move_hit(battle_context, move_context) {
            return;
        }

        let num_hits = if move_context.pokemove.is_multi_hit {
            BattleEngine::get_multi_hit_hits(battle_context, move_context)
        } else {
            1
        };

        for _ in 0..num_hits {
            match move_context.pokemove.category {
                MoveCategory::Status => {
                    Self::apply_status_move(battle_context, move_context);
                }
                MoveCategory::Physical | MoveCategory::Special => {
                    let damage_dealt = BattleEngine::single_hit_execution(
                        battle_context,
                        move_context,
                        turn_state,
                    );
                    Self::apply_recoil(battle_context, move_context, damage_dealt, turn_state);
                    Self::apply_secondary_effect(battle_context, move_context);
                }
            };

            // cancel if either pokemon faints
            if battle_context
                .battle_state
                .get_active_pokemon(move_context.target_trainer)
                .is_fainted()
                || battle_context
                    .battle_state
                    .get_active_pokemon(move_context.src_trainer)
                    .is_fainted()
            {
                break;
            }
        }

        if move_context.move_name != MoveName::Struggle {
            Self::deduct_pp(battle_context, move_context);
        }
    }

    // returns damage dealt
    fn single_hit_execution(
        battle_context: &mut BattleContext,
        move_context: &MoveContext,
        turn_state: &mut TurnState,
    ) -> u32 {
        let damage = BattleEngine::calculate_damage(battle_context, move_context);
        BattleEngine::deal_damage(battle_context, Some(move_context), None, turn_state, damage)
    }

    fn deduct_pp(battle_context: &mut BattleContext, move_context: &MoveContext) {
        let mut deduct_pp_query =
            Query::GetDeductPP(PayloadMoveQuery::u8_with_default(*move_context, 1));
        battle_context
            .query_bus
            .query(&mut deduct_pp_query, battle_context.battle_state);
        let pp_to_deduct = deduct_pp_query.into_payload_move_query().get_u8();

        battle_context
            .battle_state
            .get_active_pokemon_mut(move_context.src_trainer)
            .deduct_pp(&move_context.move_name, pp_to_deduct);
    }

    pub fn deal_damage_and_heal(
        battle_context: &mut BattleContext,
        target_damage_trainer: bool,
        target_heal_trainer: bool,
        damage: u32,
        turn_state: &mut TurnState,
    ) {
        let damage_dealt = Self::deal_damage(
            battle_context,
            None,
            Some(target_damage_trainer),
            turn_state,
            damage,
        );
        Self::heal(
            battle_context,
            target_heal_trainer,
            damage_dealt,
            turn_state,
        );
    }

    pub fn heal(
        battle_context: &mut BattleContext,
        target_trainer: bool,
        heal_amt: u32,
        turn_state: &mut TurnState,
    ) {
        battle_context
            .battle_state
            .get_side_mut(target_trainer)
            .heal(heal_amt);
    }

    // returns damage dealt
    pub fn deal_damage(
        battle_context: &mut BattleContext,
        move_context: Option<&MoveContext>,
        target_trainer: Option<bool>,
        turn_state: &mut TurnState,
        damage: u32,
    ) -> u32 {
        if move_context.is_none() || target_trainer.is_none() {
            panic!("Move context and target trainer must be provided to deal damage");
        }

        let mut final_damage_query = if move_context.is_some() {
            Query::FinalDamage(FinalDamageQuery::from_move(damage, *move_context.unwrap()))
        } else {
            Query::FinalDamage(FinalDamageQuery::from_non_move(
                damage,
                target_trainer.unwrap(),
            ))
        };
        battle_context
            .query_bus
            .query(&mut final_damage_query, battle_context.battle_state);

        let target = if move_context.is_some() {
            move_context.unwrap().target_trainer
        } else {
            target_trainer.unwrap()
        };

        let final_damage = final_damage_query.into_final_damage_query().damage;
        let (caused_faint, damage_dealt) = battle_context
            .battle_state
            .get_side_mut(target)
            .take_damage(final_damage);

        if caused_faint {
            let faint_event = Event::Faint(FaintEvent {
                move_context: move_context.copied(),
                trainer_side: target,
            });
            battle_context.event_queue.add_event(faint_event);
            battle_context
                .battle_state
                .get_active_pokemon_mut(target)
                .set_fainted();
            turn_state.record_faint(target);
        }

        damage_dealt
    }

    fn apply_status_move(battle_context: &mut BattleContext, move_context: &MoveContext) {
        if let Some(status) = &move_context.pokemove.status {
            BattleEngine::set_status(battle_context, move_context, *status);
        }

        if let Some(volatile_status) = &move_context.pokemove.volatile_status {
            Self::set_volatile_status(battle_context, move_context, *volatile_status);
        }

        if let Some(boosts) = &move_context.pokemove.boosts {
            for (stat, amount) in boosts {
                Self::apply_boost(battle_context, move_context.target_trainer, *stat, *amount);
            }
        }
    }

    fn apply_secondary_effect(battle_context: &mut BattleContext, move_context: &MoveContext) {
        if move_context.pokemove.secondary_effects.is_none() {
            return;
        }

        let target_fainted = battle_context
            .battle_state
            .get_active_pokemon(move_context.target_trainer)
            .is_fainted();
        let source_fainted = battle_context
            .battle_state
            .get_active_pokemon(move_context.src_trainer)
            .is_fainted();

        let secondary_effects = move_context.pokemove.secondary_effects.as_ref().unwrap();

        secondary_effects.iter().for_each(|chance_and_effects| {
            let (chance, effects) = chance_and_effects;

            let mut chance_query = Query::OnSecondaryEffectChance(
                PayloadMoveQuery::u8_with_default(*move_context, *chance),
            );
            battle_context
                .query_bus
                .query(&mut chance_query, battle_context.battle_state);
            let modified_chance = chance_query.into_payload_move_query().get_u8();

            let roll = battle_context
                .battle_state
                .get_random_check(modified_chance as u32, 100);
            if !roll {
                return;
            }

            for effect in effects {
                if (effect.affect_target() && target_fainted)
                    || (effect.affect_source() && source_fainted)
                {
                    continue;
                }

                match effect {
                    SecondaryEffect::Status(status) => {
                        Self::set_status(battle_context, move_context, *status);
                    }
                    SecondaryEffect::VolatileStatus(volatile_status) => {
                        Self::set_volatile_status(battle_context, move_context, *volatile_status);
                    }
                    SecondaryEffect::UserBoost(stat, amount) => {
                        Self::apply_boost(battle_context, move_context.src_trainer, *stat, *amount);
                    }
                    SecondaryEffect::TargetBoost(stat, amount) => {
                        Self::apply_boost(
                            battle_context,
                            move_context.target_trainer,
                            *stat,
                            *amount,
                        );
                    }
                }
            }
        });
    }

    fn apply_recoil(
        battle_context: &mut BattleContext,
        move_context: &MoveContext,
        damage_dealt: u32,
        turn_state: &mut TurnState,
    ) {
        if move_context.move_name == MoveName::Struggle {
            let recoil_damage = battle_context
                .battle_state
                .get_active_pokemon(move_context.src_trainer)
                .pokemon
                .max_hp
                / 4;

            Self::deal_damage(
                battle_context,
                None,
                Some(move_context.src_trainer),
                turn_state,
                recoil_damage as u32,
            );
        } else if move_context.pokemove.recoil.is_some() {
            let percent = move_context.pokemove.recoil.unwrap();
            let base_recoil_damage = (damage_dealt * percent as u32) / 100;

            // TODO: account for abilities that affect recoil damage
            Self::deal_damage(
                battle_context,
                None,
                Some(move_context.src_trainer),
                turn_state,
                base_recoil_damage,
            );
        }
    }

    fn apply_boost(
        battle_context: &mut BattleContext,
        target_trainer: bool,
        stat: BoostableStat,
        amount: i8,
    ) {
        battle_context
            .battle_state
            .get_active_pokemon_mut(target_trainer)
            .modify_boost(stat, amount);
    }

    fn set_volatile_status(
        battle_context: &mut BattleContext,
        move_context: &MoveContext,
        volatile_status: VolatileStatus,
    ) {
        // FIXME: Always assumes target trainer is target of status currently
        let target = move_context.target_trainer;

        let mut can_apply_volatile_status_query = Query::CanApplyVolatileStatus(
            CanApplyVolatileStatusQuery::new(*move_context, target, volatile_status),
        );
        battle_context.query_bus.query(
            &mut can_apply_volatile_status_query,
            battle_context.battle_state,
        );
        if !can_apply_volatile_status_query
            .into_can_apply_volatile_status_query()
            .can_apply
        {
            return;
        }

        let volatile_status_handler = battle_context
            .battle_state
            .get_active_pokemon_mut(target)
            .add_volatile_status(volatile_status);

        BattleEngine::register_handler(
            volatile_status_handler,
            &mut battle_context.event_registry,
            &mut battle_context.query_bus.registry,
        );
    }

    pub fn queue_after_turn_effects(
        battle_context: &mut BattleContext,
        turn_state: &mut TurnState,
    ) {
        let trainer_1_first = Self::resolve_speed_order(battle_context);

        battle_context
            .event_queue
            .add_event(Event::OnTurnEnd(trainer_1_first));
        battle_context
            .event_queue
            .add_event(Event::OnTurnEnd(!trainer_1_first));
    }

    pub fn set_status(
        battle_context: &mut BattleContext,
        move_context: &MoveContext,
        status: Status,
    ) {
        // FIXME: Always assumes that the target trainer is target of status.
        let status_target = move_context.target_trainer;

        let mut can_apply_status_query = Query::CanApplyStatus(CanApplyStatusQuery::new(
            *move_context,
            status_target,
            status,
        ));
        battle_context
            .query_bus
            .query(&mut can_apply_status_query, battle_context.battle_state);
        if !can_apply_status_query
            .into_can_apply_status_query()
            .can_apply
        {
            return;
        }

        if battle_context
            .battle_state
            .get_active_pokemon(status_target)
            .status
            != None
        {
            panic!("Trying to apply status to pokemon that already has a status");
        }

        // let prev_status_handler = &battle_context
        //     .battle_state
        //     .get_active_pokemon(move_context.target_trainer)
        //     .status_handler;
        // match prev_status_handler {
        //     Some(handler) => {
        //         BattleEngine::unregister_handler(
        //             &handler,
        //             &mut battle_context.event_registry,
        //             &mut battle_context.query_bus.registry,
        //         );
        //     }
        //     None => {}
        // }

        battle_context
            .battle_state
            .get_active_pokemon_mut(status_target)
            .set_status(status);

        let new_status_handler = &battle_context
            .battle_state
            .get_active_pokemon(status_target)
            .status_handler;
        match new_status_handler {
            Some(handler) => {
                BattleEngine::register_handler(
                    &handler,
                    &mut battle_context.event_registry,
                    &mut battle_context.query_bus.registry,
                );
            }
            None => panic!("Status handler should be set after setting status"),
        }
    }

    pub fn remove_status(battle_context: &mut BattleContext, trainer: bool) {
        let prev_status_handler = &battle_context
            .battle_state
            .get_active_pokemon(trainer)
            .status_handler;
        match prev_status_handler {
            Some(handler) => {
                BattleEngine::unregister_handler(
                    &handler,
                    &mut battle_context.event_registry,
                    &mut battle_context.query_bus.registry,
                );
            }
            None => panic!("No status handler to unregister when removing status"),
        }

        battle_context
            .battle_state
            .get_active_pokemon_mut(trainer)
            .clear_status();
    }

    pub fn remove_volatile_status(
        battle_context: &mut BattleContext,
        trainer: bool,
        volatile_status: VolatileStatus,
    ) {
        let pokemon_battle_instance = battle_context.battle_state.get_active_pokemon(trainer);

        let volatile_status_handler =
            pokemon_battle_instance.get_volatile_status_handler(&volatile_status);

        BattleEngine::unregister_handler(
            &volatile_status_handler,
            &mut battle_context.event_registry,
            &mut battle_context.query_bus.registry,
        );

        battle_context
            .battle_state
            .get_active_pokemon_mut(trainer)
            .remove_volatile_status(&volatile_status);
    }

    fn calculate_damage(battle_context: &mut BattleContext, move_context: &MoveContext) -> u32 {
        let mut base_power_query = Query::OnBasePower(PayloadMoveQuery::vec_f32(*move_context));
        battle_context
            .query_bus
            .query(&mut base_power_query, battle_context.battle_state);
        let modified_base_power = base_power_query
            .into_payload_move_query()
            .as_combined_modifier();

        let (modified_atk, modified_def) = match move_context.pokemove.category {
            MoveCategory::Physical => (
                BattleEngine::get_effective_stat_value(
                    battle_context,
                    move_context.src_trainer,
                    StatEnum::Attack,
                ),
                BattleEngine::get_effective_stat_value(
                    battle_context,
                    move_context.target_trainer,
                    StatEnum::Defense,
                ),
            ),
            MoveCategory::Special => (
                BattleEngine::get_effective_stat_value(
                    battle_context,
                    move_context.src_trainer,
                    StatEnum::SpecialAttack,
                ),
                BattleEngine::get_effective_stat_value(
                    battle_context,
                    move_context.target_trainer,
                    StatEnum::SpecialDefense,
                ),
            ),
            MoveCategory::Status => panic!("Cannot calculate damage for Status move"),
        };

        let mut mod1_query = Query::OnMod1(PayloadMoveQuery::vec_f32(*move_context));
        battle_context
            .query_bus
            .query(&mut mod1_query, battle_context.battle_state);
        let mod1 = mod1_query
            .into_payload_move_query()
            .as_combined_modifier_default(1);

        let mut mod2_query = Query::OnMod2(PayloadMoveQuery::vec_f32(*move_context));
        battle_context
            .query_bus
            .query(&mut mod2_query, battle_context.battle_state);
        let mod2 = mod2_query
            .into_payload_move_query()
            .as_combined_modifier_default(1);

        let mut mod3_query = Query::OnMod3(PayloadMoveQuery::vec_f32(*move_context));
        battle_context
            .query_bus
            .query(&mut mod3_query, battle_context.battle_state);
        let mod3 = mod3_query
            .into_payload_move_query()
            .as_combined_modifier_default(1);

        let mut is_crit_query = Query::IsCrit(PayloadMoveQuery::bool(*move_context));
        battle_context
            .query_bus
            .query(&mut is_crit_query, battle_context.battle_state);
        let is_crit = is_crit_query.into_payload_move_query().get_bool();

        let crit_mult: f32 = if !is_crit {
            1.0
        } else {
            let mut crit_mult_query =
                Query::CritMult(PayloadMoveQuery::f32_with_default(*move_context, 1.5));
            battle_context
                .query_bus
                .query(&mut crit_mult_query, battle_context.battle_state);
            crit_mult_query.into_payload_move_query().get_f32()
        };

        let r = (100 - battle_context.battle_state.get_rand_num(16));

        let stab_mult: f32 = if !Self::is_stab(battle_context, move_context) {
            1.0
        } else {
            let mut stab_mult_query =
                Query::StabMult(PayloadMoveQuery::f32_with_default(*move_context, 1.5));
            battle_context
                .query_bus
                .query(&mut stab_mult_query, battle_context.battle_state);
            stab_mult_query.into_payload_move_query().get_f32()
        };

        let move_type = move_context.pokemove.move_type;
        let (type1_mult, type2_mult) = match battle_context
            .battle_state
            .get_active_pokemon(move_context.target_trainer)
            .pokemon
            .typing
        {
            PokemonTyping::MonoType(t) => {
                (effectiveness::type_effectiveness(move_type, t), 1.0 as f32)
            }
            PokemonTyping::DualType(t1, t2) => (
                effectiveness::type_effectiveness(move_type, t1),
                effectiveness::type_effectiveness(move_type, t2),
            ),
        };

        damage_utils::get_damage_for_move(
            battle_context
                .battle_state
                .get_active_pokemon(move_context.src_trainer)
                .pokemon
                .level,
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

    fn is_stab(battle_context: &mut BattleContext, move_context: &MoveContext) -> bool {
        let pokemon = &battle_context
            .battle_state
            .get_active_pokemon(move_context.src_trainer)
            .pokemon;
        let move_type = move_context.pokemove.move_type;

        match pokemon.typing {
            PokemonTyping::DualType(a, b) => a == move_type || b == move_type,
            PokemonTyping::MonoType(a) => a == move_type,
        }
    }

    fn get_default_immunity(
        battle_context: &mut BattleContext,
        move_context: &MoveContext,
    ) -> bool {
        let pokemon = &battle_context
            .battle_state
            .get_active_pokemon(move_context.target_trainer)
            .pokemon;
        let move_type = move_context.pokemove.move_type;

        match pokemon.typing {
            PokemonTyping::DualType(a, b) => {
                effectiveness::type_immunity(move_type, a)
                    || effectiveness::type_immunity(move_type, b)
            }
            PokemonTyping::MonoType(a) => effectiveness::type_immunity(move_type, a),
        }
    }

    pub fn resolve_speed_order(battle_context: &mut BattleContext) -> bool {
        let speed1 = BattleEngine::get_effective_stat_value(battle_context, true, StatEnum::Speed);
        let speed2 = BattleEngine::get_effective_stat_value(battle_context, false, StatEnum::Speed);
        if speed1 > speed2 {
            true
        } else if speed2 > speed1 {
            false
        } else {
            battle_context.battle_state.get_random_check(1, 2)
        }
    }

    fn check_move_execution(
        battle_context: &mut BattleContext,
        move_context: &MoveContext,
    ) -> bool {
        let mut try_use_move_query = Query::TryUseMove(TryUseMoveQuery::new(*move_context));
        battle_context
            .query_bus
            .query(&mut try_use_move_query, battle_context.battle_state);
        let try_use_move_payload = try_use_move_query.into_try_use_move_query();
        BattleEngine::process_try_use_move_secondary_effects(
            battle_context,
            move_context,
            &try_use_move_payload,
        );
        if try_use_move_payload.should_cancel {
            return false;
        }

        let mut invuln_query =
            Query::CheckInvulnerability(PayloadMoveQuery::bool_with_default(*move_context, true));
        battle_context
            .query_bus
            .query(&mut invuln_query, battle_context.battle_state);
        if !invuln_query.into_payload_move_query().get_bool() {
            return false;
        }

        let mut immunity_query = Query::CheckImmunity(PayloadMoveQuery::bool_with_default(
            *move_context,
            !Self::get_default_immunity(battle_context, move_context),
        ));
        battle_context
            .query_bus
            .query(&mut immunity_query, battle_context.battle_state);
        if !immunity_query.into_payload_move_query().get_bool() {
            return false;
        }

        true
    }

    fn check_move_hit(battle_context: &mut BattleContext, move_context: &MoveContext) -> bool {
        if move_context.pokemove.accuracy.is_none() {
            return true;
        }

        let mut get_move_hit_chance_query =
            Query::GetMoveHitChance(PayloadMoveQuery::vec_f32_with_default(
                *move_context,
                vec![
                    move_context.pokemove.accuracy.unwrap() as f32,
                    Self::get_accuracy_mutliplier(battle_context, move_context),
                ],
            ));
        battle_context
            .query_bus
            .query(&mut get_move_hit_chance_query, battle_context.battle_state);
        let modified_accuracy = cmp::min(
            get_move_hit_chance_query
                .into_payload_move_query()
                .as_combined_modifier(),
            100 as u32,
        ) as u8;

        let accuracy_roll = battle_context.battle_state.get_rand_num_inclusive(1, 100);
        accuracy_roll <= modified_accuracy
    }

    fn process_try_use_move_secondary_effects(
        battle_context: &mut BattleContext,
        move_context: &MoveContext,
        try_use_move_payload: &TryUseMoveQuery,
    ) {
        if try_use_move_payload.unfreeze || try_use_move_payload.wake_sleep {
            BattleEngine::remove_status(battle_context, move_context.target_trainer);
        } else if try_use_move_payload.confuse_self {
            todo!("Implement deal damage from confusion");
        } else if try_use_move_payload.unconfuse {
            BattleEngine::remove_volatile_status(
                battle_context,
                move_context.target_trainer,
                VolatileStatus::Confusion,
            );
        }
    }

    fn switch_out_pokemon(battle_context: &mut BattleContext, trainer: bool, switch_idx: usize) {
        let pokemon_battle_instance = battle_context
            .battle_state
            .get_side_mut(trainer)
            .get_active_pokemon();

        BattleEngine::unregister_handlers_for_pokemon(
            &mut battle_context.event_registry,
            &mut battle_context.query_bus.registry,
            pokemon_battle_instance,
        );
    }

    fn get_multi_hit_hits(battle_context: &mut BattleContext, move_context: &MoveContext) -> u8 {
        let mut multi_hit_range_query = Query::MultiHitRange(MultiHitRangeQuery {
            move_context: *move_context,
            min_hits: 2,
            max_hits: 5,
        });
        battle_context
            .query_bus
            .query(&mut multi_hit_range_query, battle_context.battle_state);
        let multi_hit_range_payload = multi_hit_range_query.into_multi_hit_range_query();

        let mut multi_hit_hits_query = Query::MultiHitHits(MultiHitHitsQuery {
            move_context: *move_context,
            min_hits: multi_hit_range_payload.min_hits,
            max_hits: multi_hit_range_payload.max_hits,
            num_hits: 0,
        });
        battle_context
            .query_bus
            .query(&mut multi_hit_hits_query, battle_context.battle_state);
        let multi_hit_hits_payload = multi_hit_hits_query.into_multi_hit_hits_query();
        multi_hit_hits_payload.num_hits
    }

    pub fn get_effective_stat_value(
        battle_context: &mut BattleContext,
        trainer: bool,
        stat_enum: StatEnum,
    ) -> u32 {
        let base_stat_value = battle_context
            .battle_state
            .get_active_pokemon(trainer)
            .pokemon
            .get_stat_value(stat_enum);
        let boost_value = battle_context
            .battle_state
            .get_active_pokemon(trainer)
            .boosts[BoostableStat::Stat(stat_enum)];

        let mut stat_query = Query::OnStat(OnStatQuery {
            trainer,
            stat: stat_enum,
            mults: vec![
                base_stat_value as f32,
                BattleEngine::get_stat_boost_to_multiplier(boost_value),
            ],
        });
        battle_context
            .query_bus
            .query(&mut stat_query, battle_context.battle_state);
        let stat_payload = stat_query.into_on_stat_query();
        damage_utils::rounded_damage_from_modifiers(&stat_payload.mults)
    }

    fn get_accuracy_mutliplier(
        battle_context: &mut BattleContext,
        move_context: &MoveContext,
    ) -> f32 {
        let src_accuracy_stage = battle_context
            .battle_state
            .get_active_pokemon(move_context.src_trainer)
            .boosts[BoostableStat::Accuracy];
        let target_evasion_stage = battle_context
            .battle_state
            .get_active_pokemon(move_context.target_trainer)
            .boosts[BoostableStat::Evasion];

        let overall_accuracy_stage = (src_accuracy_stage - target_evasion_stage).clamp(-6, 6);

        let percent: u16 = match overall_accuracy_stage {
            -6 => 33,
            -5 => 36,
            -4 => 43,
            -3 => 50,
            -2 => 60,
            -1 => 75,
            0 => 100,
            1 => 133,
            2 => 166,
            3 => 200,
            4 => 233,
            5 => 266,
            6 => 300,
            _ => panic!("Should not have any other stage after clamp"),
        };

        percent as f32 / 100.0
    }

    fn get_stat_boost_to_multiplier(boost: i8) -> f32 {
        if boost >= 0 {
            (2.0 + boost as f32) / 2.0
        } else {
            2.0 / (2 - boost) as f32
        }
    }

    fn register_handlers_for_pokemon(
        event_registry: &mut Registry<Event, dyn EventHandler>,
        query_registry: &mut Registry<Query, dyn QueryHandler>,
        pokemon_battle_instance: &PokemonBattleInstance,
    ) {
        event_registry.add_handlers(pokemon_battle_instance.get_all_event_handlers());
        query_registry.add_handlers(pokemon_battle_instance.get_all_query_handlers());
    }

    fn unregister_handlers_for_pokemon(
        event_registry: &mut Registry<Event, dyn EventHandler>,
        query_registry: &mut Registry<Query, dyn QueryHandler>,
        pokemon_battle_instance: &PokemonBattleInstance,
    ) {
        event_registry.remove_handlers(pokemon_battle_instance.get_all_event_handlers());
        query_registry.remove_handlers(pokemon_battle_instance.get_all_query_handlers());
    }

    fn register_handler(
        handler: &Arc<dyn CombinedHandler>,
        event_registry: &mut Registry<Event, dyn EventHandler>,
        query_registry: &mut Registry<Query, dyn QueryHandler>,
    ) {
        event_registry.add_handler(handler.clone());
        query_registry.add_handler(handler.clone());
    }

    fn unregister_handler(
        handler: &Arc<dyn CombinedHandler>,
        event_registry: &mut Registry<Event, dyn EventHandler>,
        query_registry: &mut Registry<Query, dyn QueryHandler>,
    ) {
        event_registry.remove_handler(handler.clone());
        query_registry.remove_handler(handler.clone());
    }
}
