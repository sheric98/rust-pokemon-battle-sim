use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use crate::{
    core::{
        pokemon::{boostable_stat::BoostableStat, stat_enum::StatEnum},
        pokemove::{
            move_category::{self, MoveCategory},
            move_name::MoveName,
            move_target::MoveTarget,
            secondary_effect::SecondaryEffect,
        },
        poketype::poketype::PokeType,
        status::{status::Status, volatile_status::VolatileStatus},
    },
    event::event_handler::EventHandler,
};

#[derive(TypedBuilder)]
pub struct PokeMove {
    pub name: MoveName,
    #[builder(default)]
    pub power: Option<u32>,
    #[builder(default)]
    pub accuracy: Option<u8>,
    #[builder(default = 0)]
    pub priority: i8,
    pub move_type: PokeType,
    pub category: MoveCategory,
    pub pp: u8,
    #[builder(default = false)]
    pub is_multi_hit: bool,

    #[builder(default)]
    pub secondary_effects: Option<Vec<(u8, Vec<SecondaryEffect>)>>,

    #[builder(default)]
    pub status: Option<Status>,
    #[builder(default)]
    pub volatile_status: Option<VolatileStatus>,
    #[builder(default)]
    pub boosts: Option<Vec<(BoostableStat, i8)>>,

    #[builder(default=MoveTarget::Opponent)]
    pub target: MoveTarget,
}
