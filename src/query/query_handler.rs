use crate::{battle::state::BattleState, common::subscriber::Subscriber, query::query::Query};

pub trait QueryHandler: Subscriber<Query> {
    fn handle(&self, query: &Query, battle_state: &BattleState);
}
