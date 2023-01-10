use serde::{Deserialize, Serialize};

/// Describes a list of formats that a Card is legal to be your Commander in play formats that utilize Commanders.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(missing_docs)]
pub struct LeadershipSkills {
    pub brawl: bool,
    pub commander: bool,
    pub oathbreaker: bool,
}
