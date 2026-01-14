use enum_map::Enum;
use strum::{Display, EnumIter};

#[derive(Clone, Copy, Enum, EnumIter, Default, Hash, Eq, PartialEq, Display)]
pub enum StatEnum {
    #[default] // this should never be used as a default, but EnumMap requires it
    HP,
    Attack,
    Defense,
    SpecialAttack,
    SpecialDefense,
    Speed,
}
