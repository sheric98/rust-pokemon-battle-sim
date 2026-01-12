use crate::{
    common::{context::MoveContext, has_kind::HasKind},
    core::pokemove::move_name::MoveName,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum EventKind {
    Damage,
    Switch,
    BeginTurn,
    Faint,
    OnTurnEnd,
}

pub enum Event {
    Damage(DamageEvent),
    Switch(SwitchEvent),
    Faint(FaintEvent),
    BeginTurn,
    OnTurnEnd,
}

impl HasKind for Event {
    type Kind = EventKind;

    fn kind(&self) -> EventKind {
        match self {
            Event::Damage(_) => EventKind::Damage,
            Event::Switch(_) => EventKind::Switch,
            Event::BeginTurn => EventKind::BeginTurn,
            Event::Faint(_) => EventKind::Faint,
            Event::OnTurnEnd => EventKind::OnTurnEnd,
        }
    }
}

impl Event {}

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

pub struct FaintEvent {
    pub move_context: Option<MoveContext>,
    pub trainer_side: bool,
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
