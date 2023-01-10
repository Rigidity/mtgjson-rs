use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::{booster_config::BoosterConfig, booster_sheet::BoosterSheet};

/// Information about a booster.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BoosterInfo {
    /// Booster pack configurations.
    pub boosters: Vec<BoosterConfig>,

    /// Sum of all booster configurations weights.
    pub boosters_total_weight: u32,

    /// All possible sheets of cards to use within booster packs.
    pub sheets: HashMap<String, BoosterSheet>,
}
