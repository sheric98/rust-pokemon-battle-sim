use enum_map::Enum;
use strum::Display;
use strum::EnumIter;

use crate::core::pokemon::stat_enum::StatEnum;

#[derive(Enum, EnumIter, Hash, Eq, PartialEq, Display)]
pub enum BoostableStat {
    Stat(StatEnum),
    Accuracy,
    Evasion,
}
