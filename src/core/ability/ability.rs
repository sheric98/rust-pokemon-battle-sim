use enum_map::Enum;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Enum, Serialize, Deserialize)]
pub enum Ability {
    Blaze,
    Overgrow,
    Torrent,
}
