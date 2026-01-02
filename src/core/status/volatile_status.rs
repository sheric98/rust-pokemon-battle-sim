use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum VolatileStatus {
    Confusion,
    Infatuation,
    LeechSeed,
}
