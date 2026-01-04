#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MoveSlot {
    Slot0,
    Slot1,
    Slot2,
    Slot3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SwitchSlot {
    Slot0,
    Slot1,
    Slot2,
    Slot3,
    Slot4,
    Slot5,
}

pub enum Action {
    Move(MoveSlot),
    Switch(SwitchSlot),
}

impl Action {
    pub fn is_move(&self) -> bool {
        matches!(self, Action::Move(_))
    }

    pub fn is_switch(&self) -> bool {
        matches!(self, Action::Switch(_))
    }
}
