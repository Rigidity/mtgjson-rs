use serde::{Deserialize, Serialize};

/// Related card information.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct RelatedCards {
    /// Reverse related cards.
    pub reverse_related: Option<Vec<String>>,

    /// Spellbook related cards.
    pub spellbook: Option<Vec<String>>,
}
