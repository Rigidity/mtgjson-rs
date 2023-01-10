use std::collections::HashMap;

use serde::{Deserialize, Serialize};

mod booster_config;
mod booster_info;
mod booster_sheet;
mod named_booster_info;

pub use booster_config::*;
pub use booster_info::*;
pub use booster_sheet::*;
pub use named_booster_info::*;

/// Represents the booster packs for a given set.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Booster {
    /// Possible configurations in a traditional booster pack.
    pub default: Option<BoosterInfo>,

    /// Used for alternative booster packs.
    #[serde(flatten)]
    pub boosters: HashMap<String, NamedBoosterInfo>,
}
