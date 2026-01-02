use crate::core::{
    pokemon::stat_enum::StatEnum,
    status::{status::Status, volatile_status::VolatileStatus},
};

pub enum SecondaryEffect {
    Status(Status),
    VolatileStatus(VolatileStatus),
    UserBoost(StatEnum, i8),
    TargetBoost(StatEnum, i8),
}
