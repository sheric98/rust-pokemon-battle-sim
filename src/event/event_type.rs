use crate::{common::has_kind::HasKind, core::pokemove::move_name::MoveName};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum EventKind {
    Damage,
    Switch,
    BeginTurn,
    OnBasePower,
    OnAtk,
    OnDef,
    OnMod1,
    OnMod2,
    OnMod3,
    IsCrit,
    CritMult,
    StabMult,
    OnPriority,
    BeforeMove,
    GetDeductPP,
    CheckInvulnerability,
    CheckImmunity,
    TryHit,
}

pub enum EventType {
    Damage(DamageEvent),
    Switch(SwitchEvent),
    BeginTurn,
}

impl HasKind for EventType {
    type Kind = EventKind;

    fn kind(&self) -> EventKind {
        match self {
            EventType::Damage(_) => EventKind::Damage,
            EventType::Switch(_) => EventKind::Switch,
            EventType::BeginTurn => EventKind::BeginTurn,
        }
    }
}

impl EventType {

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
