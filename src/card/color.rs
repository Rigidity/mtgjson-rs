use serde::{Deserialize, Serialize};

/// The color of a card.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[allow(missing_docs)]
pub enum Color {
    #[serde(rename = "B")]
    Black,

    #[serde(rename = "G")]
    Green,

    #[serde(rename = "R")]
    Red,

    #[serde(rename = "U")]
    Blue,

    #[serde(rename = "W")]
    White,
}
