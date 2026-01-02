use crate::{common::has_kind::HasKind, query::payload::PayloadMoveQuery};

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub enum QueryKind {
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

pub enum Query {
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
    GetDeductPP(PayloadMoveQuery),
    CheckInvulnerability(PayloadMoveQuery),
    CheckImmunity(PayloadMoveQuery),
    TryHit(PayloadMoveQuery),
    FinalDamage(PayloadMoveQuery),
}

impl HasKind for Query {
    type Kind = QueryKind;

    fn kind(&self) -> QueryKind {
        todo!()
    }
}

impl Query {
    pub fn into_payload_move_query(self) -> PayloadMoveQuery {
        match self {
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
        }
    }
}
