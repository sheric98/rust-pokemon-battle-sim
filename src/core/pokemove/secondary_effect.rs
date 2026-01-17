use crate::core::{
    pokemon::{boostable_stat::BoostableStat, stat_enum::StatEnum},
    status::{status::Status, volatile_status::VolatileStatus},
};

pub enum SecondaryEffect {
    Status(Status),
    VolatileStatus(VolatileStatus),
    UserBoost(BoostableStat, i8),
    TargetBoost(BoostableStat, i8),
}

impl SecondaryEffect {
    pub fn affect_target(&self) -> bool {
        match self {
            SecondaryEffect::Status(_) => true,
            SecondaryEffect::VolatileStatus(_) => true,
            SecondaryEffect::UserBoost(_, _) => false,
            SecondaryEffect::TargetBoost(_, _) => true,
        }
    }

    pub fn affect_source(&self) -> bool {
        return !self.affect_target();
    }
}
