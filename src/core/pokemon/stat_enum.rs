use enum_map::Enum;
use strum::{Display, EnumIter};

#[derive(Enum, EnumIter, Hash, Eq, PartialEq, Display)]
pub enum StatEnum {
    HP,
    Attack,
    Defense,
    SpecialAttack,
    SpecialDefense,
    Speed,
}