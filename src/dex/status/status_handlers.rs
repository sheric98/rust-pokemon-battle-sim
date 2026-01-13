use std::sync::Arc;

use crate::{
    core::{pokemove::move_category::MoveCategory, status::status::Status},
    dex::combined_handler::CombinedHandler,
    event::event_handler_effect::EventHandlerEffect,
    handler,
};

handler!(BurnHandler ( s, state ) {
    events {
        OnTurnEnd => {
            let target_trainer = s.trainer_side;
            let target_pokemon = &state.get_active_pokemon(target_trainer).pokemon;

            let burn_damage = target_pokemon.max_hp as u32 / 16;

            vec![EventHandlerEffect::Damage(burn_damage)]
        }
    },
    queries {
        OnMod1( payload ) [priority=1] => {
            let src_trainer = payload.context.src_trainer;
            let src_pokemon = &state.get_active_pokemon(src_trainer).pokemon;
            let move_category = payload.context.pokemove.category;
            // TODO: Check GUTS ability when implemented
            if move_category == MoveCategory::Physical {
                payload.get_vec_f32().push(0.5);
            }
        }
    }
});

handler!(ParalyzeHandler ( s, state ) {
    queries {
        TryUseMove( payload ) [priority=5] => {
            let src_trainer = payload.move_context.src_trainer;
            if src_trainer != s.trainer_side {
                return;
            }
            if payload.should_cancel {
                return;
            }

            let should_cancel = state.get_random_check(1, 4);
            payload.should_cancel = should_cancel;
        },
        OnSpd( payload ) [priority=1] => {
            let src_trainer = payload.context.src_trainer;
            if src_trainer != s.trainer_side {
                return;
            }

            payload.get_vec_f32().push(0.5);
        }
    }
});

handler!(SleepHandler ( s, state ) {
    queries {
        TryUseMove( payload ) [priority=0] => {
            let src_trainer = payload.move_context.src_trainer;
            if src_trainer != s.trainer_side {
                return;
            }
            if payload.should_cancel {
                return;
            }

            let new_sleep_turns = state.get_active_pokemon(src_trainer).sleep_turns - 1;
            if new_sleep_turns > 0 {
                payload.should_cancel = true;
            } else if new_sleep_turns == 0 {
                payload.wake_sleep = true;
            }

            state.get_active_pokemon_mut(src_trainer).sleep_turns = new_sleep_turns;
        }
    }
});

handler!(PoisonHandler ( s, state ) {
    events {
        OnTurnEnd => {
            let target_trainer = s.trainer_side;
            let target_pokemon = &state.get_active_pokemon(target_trainer).pokemon;

            let poison_damage = target_pokemon.max_hp as u32 / 8;

            vec![EventHandlerEffect::Damage(poison_damage)]
        }
    }
});

handler!(BadlyPoisonHandler ( s, state ) {
    events {
        OnTurnEnd => {
            let target_trainer = s.trainer_side;
            let target_max_hp = state.get_active_pokemon(target_trainer).pokemon.max_hp;
            let curr_badly_poison_turns = state.get_active_pokemon(target_trainer).badly_poison_turns;
            let new_badly_poison_turns = std::cmp::min(curr_badly_poison_turns + 1, 15);

            state.get_active_pokemon_mut(target_trainer).badly_poison_turns = new_badly_poison_turns;

            let poison_damage = (target_max_hp * new_badly_poison_turns as u16) as u32 / 8;

            vec![EventHandlerEffect::Damage(poison_damage)]
        }
    }
});

handler!(FrozenHandler ( s, state ) {
    queries {
        TryUseMove( payload ) [priority=1] => {
            let src_trainer = payload.move_context.src_trainer;
            if src_trainer != s.trainer_side {
                return;
            }
            if payload.should_cancel {
                return;
            }

            let should_unfreeze = state.get_random_check(1, 5);
            if should_unfreeze {
                payload.unfreeze = true;
            } else {
                payload.should_cancel = true;
            }
        }
    }
});

pub fn get_status_handler(status: Status, trainer: bool) -> Arc<dyn CombinedHandler> {
    match status {
        Status::Burn => Arc::new(BurnHandler {
            trainer_side: trainer,
        }),
        Status::Paralyze => Arc::new(ParalyzeHandler {
            trainer_side: trainer,
        }),
        Status::Sleep => Arc::new(SleepHandler {
            trainer_side: trainer,
        }),
        Status::Poison => Arc::new(PoisonHandler {
            trainer_side: trainer,
        }),
        Status::BadlyPoison => Arc::new(BadlyPoisonHandler {
            trainer_side: trainer,
        }),
        Status::Frozen => Arc::new(FrozenHandler {
            trainer_side: trainer,
        }),
        Status::Faint => panic!("Faint status should not have a handler"),
    }
}
