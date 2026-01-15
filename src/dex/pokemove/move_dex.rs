use std::{collections::HashMap, sync::LazyLock};

use enum_map::{EnumMap, enum_map};

use crate::core::{
    pokemon::{boostable_stat::BoostableStat, stat_enum::StatEnum},
    pokemove::{
        move_category::MoveCategory, move_name::MoveName, pokemove::PokeMove,
        secondary_effect::SecondaryEffect,
    },
    poketype::poketype::PokeType,
    status::status::Status,
};

static MOVES_DB: LazyLock<EnumMap<MoveName, PokeMove>> = LazyLock::new(|| {
    enum_map! {
        MoveName::Bite =>
            PokeMove::builder()
                .name(MoveName::Bite)
                .category(MoveCategory::Physical)
                .power(Some(60))
                .accuracy(Some(100))
                .move_type(PokeType::Dark)
                .pp(25)
                .build(),
        MoveName::Ember =>
            PokeMove::builder()
                .name(MoveName::Ember)
                .category(MoveCategory::Special)
                .power(Some(40))
                .accuracy(Some(100))
                .secondary_effects(Some(vec![(10, vec![SecondaryEffect::Status(Status::Burn)])]))
                .move_type(PokeType::Fire)
                .pp(25)
                .build(),
        MoveName::Growl =>
            PokeMove::builder()
                .name(MoveName::Growl)
                .category(MoveCategory::Status)
                .accuracy(Some(100))
                .boosts(Some(vec![(BoostableStat::Stat(StatEnum::Attack), -1)]))
                .move_type(PokeType::Normal)
                .pp(40)
                .build(),
        MoveName::Tackle =>
            PokeMove::builder()
                .name(MoveName::Tackle)
                .category(MoveCategory::Physical)
                .power(Some(35))
                .accuracy(Some(95))
                .move_type(PokeType::Normal)
                .pp(35)
                .build(),
        _ => panic!("Move not implemented"),
    }
});

#[inline]
pub fn get_move_data(move_name: &MoveName) -> &'static PokeMove {
    &MOVES_DB[*move_name]
}
