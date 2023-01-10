use serde::{Deserialize, Serialize};

use crate::language::Language;

/// Describes a list of properties for various Card Data Models in alternate languages.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ForeignData {
    /// The foreign name on the face of the card.
    pub face_name: Option<String>,

    /// The foreign flavor text of the card.
    pub flavor_text: Option<String>,

    /// The foreign language of card.
    pub language: Language,

    /// The foreign multiverse identifier of the card.
    pub multiverse_id: Option<u32>,

    /// The foreign name of the card.
    pub name: String,

    /// The foreign text ruling of the card.
    pub text: Option<String>,

    /// The foreign type of the card. Includes any supertypes and subtypes.
    #[serde(rename = "type")]
    pub card_type: Option<String>,
}
