use serde::{Deserialize, Serialize};

/// The border color of a card.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[allow(missing_docs)]
pub enum BorderColor {
    #[serde(rename = "black")]
    Black,

    #[serde(rename = "borderless")]
    Borderless,

    #[serde(rename = "gold")]
    Gold,

    #[serde(rename = "silver")]
    Silver,

    #[serde(rename = "white")]
    White,
}
