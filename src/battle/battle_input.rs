use crate::battle::actions::{Action, SwitchSlot};

pub enum SingleInput {
    StandardInput(Action),
    SwitchInInput(SwitchSlot),
}

pub type BattleInput = (Option<SingleInput>, Option<SingleInput>);
