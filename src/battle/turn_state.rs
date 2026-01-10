pub struct TurnState {
    pub fainted_sides: Vec<bool>, // ordered faints in this turn
}

impl TurnState {
    pub fn new() -> Self {
        Self {
            fainted_sides: vec![],
        }
    }

    pub fn record_faint(&mut self, trainer_1: bool) {
        self.fainted_sides.push(trainer_1);
    }
}
