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
            QueryKind::OnAtk,
            QueryKind::OnDef,
        ]
    }

    fn priority(&self, kind: &<Query as crate::common::has_kind::HasKind>::Kind) -> i32 {
        match kind {
            QueryKind::OnBasePower => 0,
            QueryKind::OnPriority => 0,
            QueryKind::OnAtk => 0,
            QueryKind::OnDef => 0,
            _ => panic!("Query priority in move handler for unhandled query"),
        }
    }
}

impl QueryHandler for StaticBattleHandler {
    fn handle(
        &self,
        query: &mut crate::query::query::Query,
        battle_state: &crate::battle::state::BattleState,
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
            Query::OnAtk(payload) => {
                let move_category = payload.context.pokemove.category;
                let pokemon = &battle_state
                    .get_active_pokemon(payload.context.src_trainer)
                    .pokemon;
                let stat = match move_category {
                    MoveCategory::Physical => pokemon.attack,
                    MoveCategory::Special => pokemon.spattack,
                    _ => panic!("Cannot get attack for Status move"),
                };

                payload.get_vec_f32().push(stat as f32)
            }
            Query::OnDef(payload) => {
                let move_category = payload.context.pokemove.category;
                let pokemon = &battle_state
                    .get_active_pokemon(payload.context.target_trainer)
                    .pokemon;
                let stat = match move_category {
                    MoveCategory::Physical => pokemon.defense,
                    MoveCategory::Special => pokemon.spdefense,
                    _ => panic!("Cannot get defense for Status move"),
                };

                payload.get_vec_f32().push(stat as f32)
            }
            _ => panic!("unhandled query for move handler"),
        }
    }
}
