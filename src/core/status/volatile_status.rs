use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub enum VolatileStatus {
    Confusion,
    Infatuation,
    LeechSeed,
}
