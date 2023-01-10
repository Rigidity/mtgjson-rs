use serde::{Deserialize, Serialize};

use super::booster_info::BoosterInfo;

/// Information about an alternative booster.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NamedBoosterInfo {
    /// Name of the premium booster pack.
    pub name: String,

    /// The normal booster info.
    #[serde(flatten)]
    pub booster_info: BoosterInfo,
}
