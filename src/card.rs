use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{identifiers::Identifiers, language::Language, purchase_urls::PurchaseUrls};

mod availability;
mod booster_type;
mod border_color;
mod color;
mod duel_deck;
mod finish;
mod foreign_data;
mod frame_effect;
mod frame_version;
mod layout;
mod leadership_skills;
mod legalities;
mod promo_type;
mod rarity;
mod related_cards;
mod ruling;
mod security_stamp;
mod side;

pub use availability::*;
pub use booster_type::*;
pub use border_color::*;
pub use color::*;
pub use duel_deck::*;
pub use finish::*;
pub use foreign_data::*;
pub use frame_effect::*;
pub use frame_version::*;
pub use layout::*;
pub use leadership_skills::*;
pub use legalities::*;
pub use promo_type::*;
pub use rarity::*;
pub use related_cards::*;
pub use ruling::*;
pub use security_stamp::*;
pub use side::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct AtomicCard {
    pub ascii_name: Option<String>,
    pub attraction_lights: Option<Vec<u8>>,
    pub card_parts: Option<Vec<String>>,
    pub color_identity: Vec<Color>,
    pub color_indicator: Option<Vec<Color>>,
    pub colors: Vec<Color>,
    #[deprecated]
    pub converted_mana_cost: f32,
    pub edhrec_rank: Option<u32>,
    #[deprecated]
    pub face_converted_mana_cost: Option<f32>,
    pub face_mana_value: Option<f32>,
    pub face_name: Option<String>,
    pub foreign_data: Vec<ForeignData>,
    pub hand: Option<String>,
    pub has_alternative_deck_limit: Option<bool>,
    pub identifiers: Identifiers,
    pub is_funny: Option<bool>,
    pub is_reserved: Option<bool>,
    pub keywords: Option<Vec<String>>,
    pub layout: Layout,
    pub leadership_skills: Option<LeadershipSkills>,
    pub legalities: Legalities,
    pub life: Option<String>,
    pub loyalty: Option<String>,
    pub mana_cost: Option<String>,
    pub mana_value: f32,
    pub name: String,
    pub power: Option<String>,
    pub printings: Option<Vec<String>>,
    pub purchase_urls: PurchaseUrls,
    pub rulings: Vec<Ruling>,
    pub side: Option<Side>,
    pub subtypes: Vec<String>,
    pub supertypes: Vec<String>,
    pub text: Option<String>,
    pub toughness: Option<String>,
    #[serde(rename = "type")]
    pub card_type: String,
    #[serde(rename = "types")]
    pub card_types: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetCard {
    pub artist: Option<String>,
    pub ascii_name: Option<String>,
    pub attraction_lights: Option<Vec<u8>>,
    pub availability: Vec<Availability>,
    pub booster_types: Option<Vec<BoosterType>>,
    pub border_color: BorderColor,
    pub card_parts: Option<Vec<String>>,
    pub color_identity: Vec<Color>,
    pub color_indicator: Option<Vec<Color>>,
    pub colors: Vec<Color>,
    #[deprecated]
    pub converted_mana_cost: f32,
    pub duel_deck: Option<DuelDeck>,
    pub edhrec_rank: Option<u32>,
    #[deprecated]
    pub face_converted_mana_cost: Option<f32>,
    pub face_flavor_name: Option<String>,
    pub face_mana_value: Option<f32>,
    pub face_name: Option<String>,
    pub finishes: Vec<Finish>,
    pub flavor_name: Option<String>,
    pub flavor_text: Option<String>,
    pub foreign_data: Vec<ForeignData>,
    pub frame_effects: Option<Vec<FrameEffect>>,
    pub frame_version: FrameVersion,
    pub hand: Option<String>,
    pub has_alternative_deck_limit: Option<bool>,
    pub has_content_warning: Option<bool>,
    #[deprecated]
    pub has_foil: bool,
    #[deprecated]
    pub has_non_foil: bool,
    pub identifiers: Identifiers,
    pub is_alternative: Option<bool>,
    pub is_full_art: Option<bool>,
    pub is_funny: Option<bool>,
    pub is_online_only: Option<bool>,
    pub is_oversized: Option<bool>,
    pub is_promo: Option<bool>,
    pub is_rebalanced: Option<bool>,
    pub is_reprint: Option<bool>,
    pub is_reserved: Option<bool>,
    pub is_starter: Option<bool>,
    pub is_story_spotlight: Option<bool>,
    pub is_textless: Option<bool>,
    pub is_timeshifted: Option<bool>,
    pub keywords: Option<Vec<String>>,
    pub language: Language,
    pub layout: Layout,
    pub leadership_skills: Option<LeadershipSkills>,
    pub legalities: Legalities,
    pub life: Option<String>,
    pub loyalty: Option<String>,
    pub mana_cost: Option<String>,
    pub mana_value: f32,
    pub name: String,
    pub number: String,
    pub original_printings: Option<Vec<Uuid>>,
    pub original_release_date: Option<NaiveDate>,
    pub original_text: Option<String>,
    pub original_type: Option<String>,
    pub other_face_ids: Option<Vec<Uuid>>,
    pub power: Option<String>,
    pub printings: Option<Vec<String>>,
    pub promo_types: Option<Vec<PromoType>>,
    pub purchase_urls: PurchaseUrls,
    pub rarity: Rarity,
    pub rebalanced_printings: Option<Vec<Uuid>>,
    pub related_cards: Option<RelatedCards>,
    pub reverse_related: Option<Vec<String>>,
    pub rulings: Vec<Ruling>,
    pub security_stamp: Option<SecurityStamp>,
    pub set_code: String,
    pub side: Option<Side>,
    pub signature: Option<String>,
    pub subset: Option<Vec<String>>,
    pub subtypes: Vec<String>,
    pub supertypes: Vec<String>,
    pub text: Option<String>,
    pub toughness: Option<String>,
    #[serde(rename = "type")]
    pub card_type: String,
    #[serde(rename = "types")]
    pub card_types: Vec<String>,
    pub uuid: Uuid,
    #[serde(default)]
    pub variations: Vec<Uuid>,
    pub watermark: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenCard {
    pub artist: Option<String>,
    pub ascii_name: Option<String>,
    pub availability: Vec<Availability>,
    pub booster_types: Option<Vec<BoosterType>>,
    pub border_color: BorderColor,
    pub card_parts: Option<Vec<String>>,
    pub color_identity: Vec<Color>,
    pub color_indicator: Option<Vec<Color>>,
    pub colors: Vec<Color>,
    pub face_flavor_name: Option<String>,
    pub face_name: Option<String>,
    pub finishes: Vec<Finish>,
    pub flavor_text: Option<String>,
    pub frame_effects: Option<Vec<FrameEffect>>,
    pub frame_version: FrameVersion,
    #[deprecated]
    pub has_foil: bool,
    #[deprecated]
    pub has_non_foil: bool,
    pub identifiers: Identifiers,
    pub is_full_art: Option<bool>,
    pub is_funny: Option<bool>,
    pub is_online_only: Option<bool>,
    pub is_promo: Option<bool>,
    pub is_reprint: Option<bool>,
    pub is_textless: Option<bool>,
    pub keywords: Option<Vec<String>>,
    pub language: Language,
    pub layout: Layout,
    pub loyalty: Option<String>,
    pub name: String,
    pub number: String,
    pub original_text: Option<String>,
    pub original_type: Option<String>,
    pub other_face_ids: Option<Vec<Uuid>>,
    pub power: Option<String>,
    pub promo_types: Option<Vec<PromoType>>,
    pub related_cards: Option<RelatedCards>,
    pub reverse_related: Vec<String>,
    pub security_stamp: Option<SecurityStamp>,
    pub set_code: String,
    pub side: Option<Side>,
    pub signature: Option<String>,
    pub subtypes: Vec<String>,
    pub supertypes: Vec<String>,
    pub text: Option<String>,
    pub toughness: Option<String>,
    #[serde(rename = "type")]
    pub card_type: String,
    #[serde(rename = "types")]
    pub card_types: Vec<String>,
    pub uuid: Uuid,
    pub watermark: Option<String>,
}
