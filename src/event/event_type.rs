use crate::{common::has_kind::HasKind, core::pokemove::move_name::MoveName};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum EventKind {
    Damage,
    Switch,
    BeginTurn,
}

pub enum Event {
    Damage(DamageEvent),
    Switch(SwitchEvent),
    BeginTurn,
}

impl HasKind for Event {
    type Kind = EventKind;

    fn kind(&self) -> EventKind {
        match self {
            Event::Damage(_) => EventKind::Damage,
            Event::Switch(_) => EventKind::Switch,
            Event::BeginTurn => EventKind::BeginTurn,
        }
    }
}

impl Event {

}


// Event Payloads with data outside of BattleState

pub struct DamageEvent {
    pub target_trainer_1_side: bool,
    pub damage: u32,
    pub caused_faint: bool,
}

pub struct SwitchEvent {
    pub orig_pokemon_idx: usize,
    pub new_pokemon_idx: usize,
}

pub struct OnPriorityEvent {
    pub priority: i8,
    pub move_name: MoveName,
}

pub struct BeforeMoveEvent {
    pub move_name: MoveName,
    pub should_cancel: bool,
}

pub struct CanApplyMoveEvent {
    pub move_name: MoveName,
    pub can_apply: bool,
}

pub struct GetDeductPPEvent {
    pub move_name: MoveName,
    pub amount: u8,
}
