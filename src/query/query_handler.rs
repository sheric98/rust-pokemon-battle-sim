use crate::{battle::state::BattleState, common::subscriber::Subscriber, query::query::Query};

pub trait QueryHandler: Subscriber<Query> + Send + Sync {
    fn handle(&self, query: &mut Query, battle_state: &BattleState);
}
