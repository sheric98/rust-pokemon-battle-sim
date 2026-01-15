use std::sync::Arc;

use crate::{
    core::status::volatile_status::VolatileStatus, dex::combined_handler::CombinedHandler,
    event::event_handler_effect::EventHandlerEffect, handler,
};

handler!(ConfusionHandler ( s, state ) {
    queries {
        TryUseMove( payload ) [priority=4] => {
            let src_trainer = payload.move_context.src_trainer;
            if src_trainer != s.trainer_side {
                return;
            }
            if payload.should_cancel {
                return;
            }

            let new_confusion_turns = state.get_active_pokemon(src_trainer).confusion_turns - 1;
            if new_confusion_turns > 0 {
                if state.get_random_check(1, 2) {
                    payload.should_cancel = true;
                    payload.confuse_self = true;
                }
            } else if new_confusion_turns == 0 {
                payload.unconfuse = true;
            }

            state.get_active_pokemon_mut(src_trainer).confusion_turns = new_confusion_turns;
        },
    }
});

handler!(InfatuationHandler ( s, state ) {
    queries {
        TryUseMove( payload ) [priority=6] => {
            let src_trainer = payload.move_context.src_trainer;
            if src_trainer != s.trainer_side {
                return;
            }
            if payload.should_cancel {
                return;
            }

            let should_fail = state.get_random_check(1, 2);
            payload.should_cancel = should_fail;
        },
    }
});

handler!(LeechSeedHandler ( s, state ) {
    events {
        OnTurnEnd => {
            let target_trainer = s.trainer_side;
            let target_pokemon = &state.get_active_pokemon(target_trainer).pokemon;

            let leech_seed_damage = target_pokemon.max_hp as u32 / 8;

            vec![EventHandlerEffect::DamageAndHeal(leech_seed_damage, target_trainer, !target_trainer)]
        }
    }
});

handler!(FlinchHandler ( s, state ) {
    queries {
        TryUseMove( payload ) [priority=3] => {
            let src_trainer = payload.move_context.src_trainer;
            if src_trainer != s.trainer_side {
                return;
            }
            if payload.should_cancel {
                return;
            }

            payload.should_cancel = true;
        },
    }
});

pub fn get_volatile_status_handler(
    status: VolatileStatus,
    trainer: bool,
) -> Arc<dyn CombinedHandler> {
    match status {
        VolatileStatus::Confusion => Arc::new(ConfusionHandler {
            trainer_side: trainer,
        }),
        VolatileStatus::Infatuation => Arc::new(InfatuationHandler {
            trainer_side: trainer,
        }),
        VolatileStatus::LeechSeed => Arc::new(LeechSeedHandler {
            trainer_side: trainer,
        }),
        VolatileStatus::Flinch => Arc::new(FlinchHandler {
            trainer_side: trainer,
        }),
    }
}
