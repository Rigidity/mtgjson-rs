use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// Booster pack configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BoosterConfig {
    /// Card contents of a booster pack.
    pub contents: HashMap<String, u32>,

    /// Odds of getting this configuration against other configurations.
    pub weight: u32,
}
