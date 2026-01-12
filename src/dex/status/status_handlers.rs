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
        TryUseMove( payload ) => {
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

pub fn get_status_handler(status: Status, trainer: bool) -> Arc<dyn CombinedHandler> {
    match status {
        Status::Burn => Arc::new(BurnHandler {
            trainer_side: trainer,
        }),
        Status::Paralyze => Arc::new(ParalyzeHandler {
            trainer_side: trainer,
        }),
        _ => todo!(),
    }
}
