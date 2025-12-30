use crate::{common::has_kind::HasKind, query::payload::PayloadQuery};

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
    OnBasePower(PayloadQuery),
    OnAtk(PayloadQuery),
    OnDef(PayloadQuery),
    OnMod1(PayloadQuery),
    OnMod2(PayloadQuery),
    OnMod3(PayloadQuery),
    IsCrit(PayloadQuery),
    CritMult(PayloadQuery),
    StabMult(PayloadQuery),
    OnPriority(PayloadQuery),
    BeforeMove(PayloadQuery),
    GetDeductPP(PayloadQuery),
    CheckInvulnerability(PayloadQuery),
    CheckImmunity(PayloadQuery),
    TryHit(PayloadQuery),
    FinalDamage(PayloadQuery),
}

impl HasKind for Query {
    type Kind = QueryKind;
    
    fn kind(&self) -> QueryKind {
        todo!()
    }
}

impl Query {
    pub fn into_payload_query(self) -> PayloadQuery {
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
