pub enum SingleBattleRequest {
    SwitchInRequest,
    ActionRequest,
}

pub enum BattleRequest {
    Request(Option<SingleBattleRequest>, Option<SingleBattleRequest>),
    BattleEnded,
}
