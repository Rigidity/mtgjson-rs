use serde::{Deserialize, Serialize};

/// Describes a list of legalities in play formats for a Card.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[allow(missing_docs)]
pub struct Legalities {
    pub brawl: Option<String>,
    pub commander: Option<String>,
    pub duel: Option<String>,
    pub future: Option<String>,
    pub frontier: Option<String>,
    pub gladiator: Option<String>,
    pub historic: Option<String>,
    pub historic_brawl: Option<String>,
    pub legacy: Option<String>,
    pub modern: Option<String>,
    pub old_school: Option<String>,
    pub pauper: Option<String>,
    pub pauper_commander: Option<String>,
    pub penny: Option<String>,
    pub pioneer: Option<String>,
    pub premodern: Option<String>,
    pub standard: Option<String>,
    pub vintage: Option<String>,
}
