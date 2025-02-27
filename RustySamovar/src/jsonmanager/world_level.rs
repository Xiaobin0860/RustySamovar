use serde::{Deserialize, Serialize};

#[derive(Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct WorldLevel {
    pub level: u32,
    pub monster_level: u32,
}
