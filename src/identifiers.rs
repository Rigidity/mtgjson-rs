use serde::{Deserialize, Serialize};

/// Describes a list of identifiers associated to a Card.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(missing_docs)]
pub struct Identifiers {
    pub card_kingdom_etched_id: Option<String>,
    pub card_kingdom_foil_id: Option<String>,
    pub card_kingdom_id: Option<String>,
    pub cardsphere_id: Option<String>,
    pub mcm_id: Option<String>,
    pub mcm_meta_id: Option<String>,
    pub mtg_arena_id: Option<String>,
    pub mtgo_foil_id: Option<String>,
    pub mtgo_id: Option<String>,
    pub mtgjson_v4_id: Option<String>,
    pub multiverse_id: Option<String>,
    pub scryfall_id: Option<String>,
    pub scryfall_oracle_id: Option<String>,
    pub scryfall_illustration_id: Option<String>,
    pub tcgplayer_product_id: Option<String>,
    pub tcgplayer_etched_product_id: Option<String>,
}
