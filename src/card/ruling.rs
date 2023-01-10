use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Describes a list of rulings for a Card.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ruling {
    /// The release date in ISO 8601 format for the rule.
    pub date: Option<NaiveDate>,

    /// The text ruling of the card.
    pub text: Option<String>,
}
