pub enum Action {
    Move1,
    Move2,
    Move3,
    Move4,
    SwitchPokemon1,
    SwitchPokemon2,
    SwitchPokemon3,
    SwitchPokemon4,
    SwitchPokemon5,
    SwitchPokemon6,
}

impl Action {
    pub fn is_move(&self) -> bool {
        matches!(
            self,
            Action::Move1 | Action::Move2 | Action::Move3 | Action::Move4
        )
    }

    pub fn is_switch(&self) -> bool {
        matches!(
            self,
            Action::SwitchPokemon1
                | Action::SwitchPokemon2
                | Action::SwitchPokemon3
                | Action::SwitchPokemon4
                | Action::SwitchPokemon5
                | Action::SwitchPokemon6
        )
    }
}

pub enum BattleAction {
    Move(u8),
    Switch(u8),
}

impl BattleAction {
    pub fn validate(self) {
        match self {
            BattleAction::Move(idx) => {
                if idx >= 4 {
                    panic!("Too large of index for move choice")
                }
            }
            BattleAction::Switch(idx) => {
                if idx >= 6 {
                    panic!("Too large of index for switch choice")
                }
            }
        }
    }
}
