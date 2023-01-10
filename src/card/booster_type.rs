use serde::{Deserialize, Serialize};

/// The type of a booster.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[allow(missing_docs)]
pub enum BoosterType {
    #[serde(rename = "deck")]
    Deck,

    #[serde(rename = "draft")]
    Draft,
}
