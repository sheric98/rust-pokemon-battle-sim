use crate::{core::pokemove::move_name::MoveName, event::payload::PayloadEvent};

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
    OnBasePower(PayloadEvent),
    OnAtk(PayloadEvent),
    OnDef(PayloadEvent),
    OnMod1(PayloadEvent),
    OnMod2(PayloadEvent),
    OnMod3(PayloadEvent),
    IsCrit(PayloadEvent),
    CritMult(PayloadEvent),
    StabMult(PayloadEvent),
    OnPriority(OnPriorityEvent),
    BeforeMove(BeforeMoveEvent),
    GetDeductPP(GetDeductPPEvent),
    CheckInvulnerability(CanApplyMoveEvent),
    CheckImmunity(CanApplyMoveEvent),
    TryHit(CanApplyMoveEvent),
}

impl EventType {
    pub fn kind(&self) -> EventKind {
        match self {
            EventType::Damage(_) => EventKind::Damage,
            EventType::Switch(_) => EventKind::Switch,
            EventType::BeginTurn => EventKind::BeginTurn,
            EventType::OnBasePower(_) => EventKind::OnBasePower,
            EventType::OnAtk(_) => EventKind::OnAtk,
            EventType::OnDef(_) => EventKind::OnDef,
            EventType::OnMod1(_) => EventKind::OnMod1,
            EventType::OnMod2(_) => EventKind::OnMod2,
            EventType::OnMod3(_) => EventKind::OnMod3,
            EventType::IsCrit(_) => EventKind::IsCrit,
            EventType::CritMult(_) => EventKind::CritMult,
            EventType::StabMult(_) => EventKind::StabMult,
            EventType::OnPriority(_) => EventKind::OnPriority,
            EventType::BeforeMove(_) => EventKind::BeforeMove,
            EventType::GetDeductPP(_) => EventKind::GetDeductPP,
            EventType::CheckInvulnerability(_) => EventKind::CheckInvulnerability,
            EventType::CheckImmunity(_) => EventKind::CheckImmunity,
            EventType::TryHit(_) => EventKind::TryHit,
        }
    }

    pub fn into_damage_event(self) -> DamageEvent {
        match self {
            EventType::Damage(e) => e,
            _ => panic!("Expected Damage event"),
        }
    }

    pub fn into_on_priority(self) -> OnPriorityEvent {
        match self {
            EventType::OnPriority(e) => e,
            _ => panic!("Expected OnPriority event"),
        }
    }

    pub fn into_before_move(self) -> BeforeMoveEvent {
        match self {
            EventType::BeforeMove(e) => e,
            _ => panic!("Expected BeforeMove event"),
        }
    }

    pub fn into_get_deduct_pp(self) -> GetDeductPPEvent {
        match self {
            EventType::GetDeductPP(e) => e,
            _ => panic!("Expected DeductPP event"),
        }
    }

    pub fn into_can_apply_move(self) -> CanApplyMoveEvent {
        match self {
            EventType::CheckInvulnerability(e) => e,
            EventType::CheckImmunity(e) => e,
            EventType::TryHit(e) => e,
            _ => panic!("Expected CanApplyMove event"),
        }
    }

    pub fn into_payload_event(self) -> PayloadEvent {
        match self {
            EventType::OnBasePower(e) => e,
            EventType::OnAtk(e) => e,
            EventType::OnDef(e) => e,
            EventType::OnMod1(e) => e,
            EventType::OnMod2(e) => e,
            EventType::OnMod3(e) => e,
            EventType::IsCrit(e) => e,
            EventType::CritMult(e) => e,
            _ => panic!("Expected Payload event"),
        }
    }
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
