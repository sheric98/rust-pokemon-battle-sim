pub enum SingleBattleRequest {
    SwitchInRequest,
    ActionRequest,
}

pub enum ActionResponse {
    SwitchInRequest(bool), // trainer_side
    BattleEnded(bool),     // winner is trainer_side
    Continue,
}

pub enum BattleRequest {
    Request(Option<SingleBattleRequest>, Option<SingleBattleRequest>),
    BattleEnded(bool), // winner is trainer_side
}
