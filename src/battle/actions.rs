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

#[derive(PartialEq, Eq)]
pub enum Action {
    Move(MoveSlot),
    Switch(SwitchSlot),
    Struggle,
}

impl Action {
    pub fn is_move(&self) -> bool {
        matches!(self, Action::Move(_))
    }

    pub fn is_switch(&self) -> bool {
        matches!(self, Action::Switch(_))
    }

    pub fn get_switch_index(&self) -> usize {
        match self {
            Action::Switch(slot) => match slot {
                SwitchSlot::Slot0 => 0,
                SwitchSlot::Slot1 => 1,
                SwitchSlot::Slot2 => 2,
                SwitchSlot::Slot3 => 3,
                SwitchSlot::Slot4 => 4,
                SwitchSlot::Slot5 => 5,
            },
            _ => panic!("Called get_switch_index on a non-switch action"),
        }
    }
}
