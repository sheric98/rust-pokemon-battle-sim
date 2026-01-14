use crate::{
    common::{context::MoveContext, has_kind::HasKind},
    query::payload::PayloadMoveQuery,
};

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub enum QueryKind {
    OnSpd,
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
    TryUseMove,
    GetDeductPP,
    CheckInvulnerability,
    CheckImmunity,
    TryHit,
    FinalDamage,
    CanApplyStatus,
}

pub enum Query {
    OnSpd(PayloadMoveQuery),
    OnBasePower(PayloadMoveQuery),
    OnAtk(PayloadMoveQuery),
    OnDef(PayloadMoveQuery),
    OnMod1(PayloadMoveQuery),
    OnMod2(PayloadMoveQuery),
    OnMod3(PayloadMoveQuery),
    IsCrit(PayloadMoveQuery),
    CritMult(PayloadMoveQuery),
    StabMult(PayloadMoveQuery),
    OnPriority(PayloadMoveQuery),
    BeforeMove(PayloadMoveQuery),
    TryUseMove(TryUseMoveQuery),
    GetDeductPP(PayloadMoveQuery),
    CheckInvulnerability(PayloadMoveQuery),
    CheckImmunity(PayloadMoveQuery),
    TryHit(PayloadMoveQuery),
    FinalDamage(PayloadMoveQuery),
    CanApplyStatus(PayloadMoveQuery),
}

impl HasKind for Query {
    type Kind = QueryKind;

    fn kind(&self) -> QueryKind {
        match self {
            Query::OnSpd(_) => QueryKind::OnSpd,
            Query::OnBasePower(_) => QueryKind::OnBasePower,
            Query::OnAtk(_) => QueryKind::OnAtk,
            Query::OnDef(_) => QueryKind::OnDef,
            Query::OnMod1(_) => QueryKind::OnMod1,
            Query::OnMod2(_) => QueryKind::OnMod2,
            Query::OnMod3(_) => QueryKind::OnMod3,
            Query::IsCrit(_) => QueryKind::IsCrit,
            Query::CritMult(_) => QueryKind::CritMult,
            Query::StabMult(_) => QueryKind::StabMult,
            Query::OnPriority(_) => QueryKind::OnPriority,
            Query::BeforeMove(_) => QueryKind::BeforeMove,
            Query::TryUseMove(_) => QueryKind::TryUseMove,
            Query::GetDeductPP(_) => QueryKind::GetDeductPP,
            Query::CheckInvulnerability(_) => QueryKind::CheckInvulnerability,
            Query::CheckImmunity(_) => QueryKind::CheckImmunity,
            Query::TryHit(_) => QueryKind::TryHit,
            Query::FinalDamage(_) => QueryKind::FinalDamage,
            Query::CanApplyStatus(_) => QueryKind::CanApplyStatus,
        }
    }
}

impl Query {
    pub fn into_payload_move_query(self) -> PayloadMoveQuery {
        match self {
            Query::OnSpd(e) => e,
            Query::OnBasePower(e) => e,
            Query::OnAtk(e) => e,
            Query::OnDef(e) => e,
            Query::OnMod1(e) => e,
            Query::OnMod2(e) => e,
            Query::OnMod3(e) => e,
            Query::IsCrit(e) => e,
            Query::CritMult(e) => e,
            Query::StabMult(e) => e,
            Query::OnPriority(e) => e,
            Query::BeforeMove(e) => e,
            Query::GetDeductPP(e) => e,
            Query::CheckInvulnerability(e) => e,
            Query::CheckImmunity(e) => e,
            Query::TryHit(e) => e,
            Query::FinalDamage(e) => e,
            Query::CanApplyStatus(e) => e,
            _ => panic!("Query is not a PayloadMoveQuery"),
        }
    }

    pub fn into_try_use_move_query(self) -> TryUseMoveQuery {
        match self {
            Query::TryUseMove(e) => e,
            _ => panic!("Query is not a TryUseMoveQuery"),
        }
    }
}

pub struct TryUseMoveQuery {
    pub move_context: MoveContext,
    pub should_cancel: bool,
    pub confuse_self: bool,
    pub unconfuse: bool,
    pub wake_sleep: bool,
    pub unfreeze: bool,
}

impl TryUseMoveQuery {
    pub fn new(move_context: MoveContext) -> Self {
        Self {
            move_context,
            should_cancel: false,
            confuse_self: false,
            unconfuse: false,
            wake_sleep: false,
            unfreeze: false,
        }
    }
}

pub struct OnSpdQuery {
    pub trainer: bool,
    pub mults: Vec<f32>,
}
