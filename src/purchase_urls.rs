use serde::{Deserialize, Serialize};

/// Describes URL paths to purchase a product from a marketplace.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(missing_docs)]
pub struct PurchaseUrls {
    pub card_kingdom: Option<String>,
    pub card_kingdom_etched: Option<String>,
    pub card_kingdom_foil: Option<String>,
    pub cardmarket: Option<String>,
    pub tcgplayer: Option<String>,
    pub tcgplayer_etched: Option<String>,
}
