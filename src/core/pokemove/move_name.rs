use enum_map::Enum;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Enum, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum MoveName {
    Empty,
    Struggle,
    Tackle,
    BulletSeed,
    Growl,
    SandAttack,
    Bite,
    Ember,
    WillOWisp,
}
