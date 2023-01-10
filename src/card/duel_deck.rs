use serde::{Deserialize, Serialize};

/// The duel deck of a card.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[allow(missing_docs)]
pub enum DuelDeck {
    #[serde(rename = "a")]
    A,

    #[serde(rename = "b")]
    B,
}
