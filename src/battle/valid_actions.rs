pub enum ValidMoves {
    Struggle,
    Moves(Vec<usize>),
}

pub struct ValidSwitches {
    pub switches: Vec<usize>,
}

pub struct ValidActions {
    pub valid_moves: ValidMoves,
    pub valid_switches: Option<ValidSwitches>,
}
