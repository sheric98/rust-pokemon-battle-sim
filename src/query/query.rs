use crate::{
    common::{context::MoveContext, has_kind::HasKind},
    core::{
        pokemon::stat_enum::StatEnum,
        status::{status::Status, volatile_status::VolatileStatus},
    },
    query::payload::PayloadMoveQuery,
};

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub enum QueryKind {
    OnStat,
    OnBasePower,
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
    GetMoveHitChance,
    FinalDamage,
    CanApplyStatus,
    CanApplyVolatileStatus,
    MultiHitRange,
    MultiHitHits,
}

pub enum Query {
    OnStat(OnStatQuery),
    OnBasePower(PayloadMoveQuery),
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
    GetMoveHitChance(PayloadMoveQuery),
    FinalDamage(PayloadMoveQuery),
    CanApplyStatus(CanApplyStatusQuery),
    CanApplyVolatileStatus(CanApplyVolatileStatusQuery),
    MultiHitRange(MultiHitRangeQuery),
    MultiHitHits(MultiHitHitsQuery),
}

impl HasKind for Query {
    type Kind = QueryKind;

    fn kind(&self) -> QueryKind {
        match self {
            Query::OnStat(_) => QueryKind::OnStat,
            Query::OnBasePower(_) => QueryKind::OnBasePower,
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
            Query::GetMoveHitChance(_) => QueryKind::GetMoveHitChance,
            Query::FinalDamage(_) => QueryKind::FinalDamage,
            Query::CanApplyStatus(_) => QueryKind::CanApplyStatus,
            Query::CanApplyVolatileStatus(_) => QueryKind::CanApplyVolatileStatus,
            Query::MultiHitRange(_) => QueryKind::MultiHitRange,
            Query::MultiHitHits(_) => QueryKind::MultiHitHits,
        }
    }
}

impl Query {
    pub fn into_payload_move_query(self) -> PayloadMoveQuery {
        match self {
            Query::OnBasePower(e) => e,
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
            Query::GetMoveHitChance(e) => e,
            Query::FinalDamage(e) => e,
            _ => panic!("Query is not a PayloadMoveQuery"),
        }
    }

    pub fn into_try_use_move_query(self) -> TryUseMoveQuery {
        match self {
            Query::TryUseMove(e) => e,
            _ => panic!("Query is not a TryUseMoveQuery"),
        }
    }

    pub fn into_on_stat_query(self) -> OnStatQuery {
        match self {
            Query::OnStat(e) => e,
            _ => panic!("Query is not an OnStatQuery"),
        }
    }

    pub fn into_multi_hit_range_query(self) -> MultiHitRangeQuery {
        match self {
            Query::MultiHitRange(e) => e,
            _ => panic!("Query is not a MultiHitRangeQuery"),
        }
    }

    pub fn into_multi_hit_hits_query(self) -> MultiHitHitsQuery {
        match self {
            Query::MultiHitHits(e) => e,
            _ => panic!("Query is not a MultiHitHitsQuery"),
        }
    }

    pub fn into_can_apply_status_query(self) -> CanApplyStatusQuery {
        match self {
            Query::CanApplyStatus(e) => e,
            _ => panic!("Query is not a CanApplyStatusQuery"),
        }
    }

    pub fn into_can_apply_volatile_status_query(self) -> CanApplyVolatileStatusQuery {
        match self {
            Query::CanApplyVolatileStatus(e) => e,
            _ => panic!("Query is not a CanApplyVolatileStatusQuery"),
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

pub struct OnStatQuery {
    pub trainer: bool,
    pub stat: StatEnum,
    pub mults: Vec<f32>,
}

pub struct CanApplyStatusQuery {
    pub move_context: MoveContext,
    pub status: Status,
    pub can_apply: bool,
}

impl CanApplyStatusQuery {
    pub fn new(move_context: MoveContext, status: Status) -> Self {
        Self {
            move_context,
            status,
            can_apply: true,
        }
    }
}

pub struct CanApplyVolatileStatusQuery {
    pub move_context: MoveContext,
    pub volatile_status: VolatileStatus,
    pub can_apply: bool,
}

impl CanApplyVolatileStatusQuery {
    pub fn new(move_context: MoveContext, volatile_status: VolatileStatus) -> Self {
        Self {
            move_context,
            volatile_status,
            can_apply: true,
        }
    }
}

pub struct MultiHitRangeQuery {
    pub move_context: MoveContext,
    pub min_hits: u8,
    pub max_hits: u8,
}

pub struct MultiHitHitsQuery {
    pub move_context: MoveContext,
    pub min_hits: u8,
    pub max_hits: u8,
    pub num_hits: u8,
}
