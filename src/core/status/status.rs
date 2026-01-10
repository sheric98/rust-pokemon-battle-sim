use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Debug)]
pub enum Status {
    Burn,
    Frozen,
    Paralyze,
    Poison,
    BadlyPoison,
    Sleep,
    Faint,
}
