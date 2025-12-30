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
        matches!(self, Action::Move1 | Action::Move2 | Action::Move3 | Action::Move4)
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

