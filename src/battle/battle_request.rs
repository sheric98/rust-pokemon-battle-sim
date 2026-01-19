use crate::battle::valid_actions::{ValidActions, ValidSwitches};

pub enum SingleBattleRequest {
    SwitchInRequest(ValidSwitches),
    ActionRequest(ValidActions),
}

pub enum ActionResponse {
    SwitchInRequest(bool), // trainer_side
    BattleEnded(Winner),   // winner is trainer_side
    Continue,
}

pub enum BattleRequest {
    Request(Option<SingleBattleRequest>, Option<SingleBattleRequest>),
    BattleEnded(Winner), // winner is trainer_side
}

pub enum Winner {
    Trainer1,
    Trainer2,
    Draw,
}
