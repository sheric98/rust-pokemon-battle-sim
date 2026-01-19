use crate::{
    common::subscriber::Subscriber,
    core::pokemove::move_category::MoveCategory,
    query::{
        payload::Payload,
        query::{Query, QueryKind},
        query_handler::QueryHandler,
    },
};

pub struct StaticBattleHandler;

impl Subscriber<Query> for StaticBattleHandler {
    fn subscriptions(&self) -> &'static [<Query as crate::common::has_kind::HasKind>::Kind] {
        &[
            QueryKind::OnPriority,
            QueryKind::OnBasePower,
            QueryKind::MultiHitHits,
            QueryKind::CanApplyStatus,
            QueryKind::CanApplyVolatileStatus,
        ]
    }

    fn priority(&self, kind: &<Query as crate::common::has_kind::HasKind>::Kind) -> i32 {
        match kind {
            QueryKind::OnBasePower => 0,
            QueryKind::OnPriority => 0,
            QueryKind::MultiHitHits => 0,
            QueryKind::CanApplyStatus => 0,
            QueryKind::CanApplyVolatileStatus => 0,
            _ => panic!("Query priority in move handler for unhandled query"),
        }
    }
}

impl QueryHandler for StaticBattleHandler {
    fn handle(
        &self,
        query: &mut crate::query::query::Query,
        battle_state: &mut crate::battle::state::BattleState,
    ) {
        match query {
            Query::OnBasePower(payload) => {
                let power = payload
                    .context
                    .pokemove
                    .power
                    .expect("Getting base power of move with no power");
                payload.get_vec_f32().push(power as f32)
            }
            Query::OnPriority(payload) => {
                payload.payload = Payload::I8(payload.context.pokemove.priority);
            }
            Query::MultiHitHits(payload) => {
                payload.num_hits =
                    battle_state.get_rand_num_inclusive(payload.min_hits, payload.max_hits);
            }
            Query::CanApplyStatus(payload) => {
                payload.can_apply = battle_state
                    .get_active_pokemon(payload.target)
                    .status
                    .is_some();
            }
            Query::CanApplyVolatileStatus(payload) => {
                payload.can_apply = !battle_state
                    .get_active_pokemon(payload.target)
                    .volatile_statuses
                    .contains_key(&payload.volatile_status);
            }
            _ => panic!("unhandled query for move handler"),
        }
    }
}
