#![deny(missing_docs)]

//! The [MTGJSON](https://mtgjson.com) API provides a repository of card information for Magic.
//! This library provides types for models from the API, for use in apps that need card information.

mod all_printings;
mod atomic_cards;
mod booster;
mod card;
mod identifiers;
mod language;
mod meta;
mod purchase_urls;
mod sealed_product;
mod set;

pub use all_printings::*;
pub use atomic_cards::*;
pub use booster::*;
pub use card::*;
pub use identifiers::*;
pub use language::*;
pub use meta::*;
pub use purchase_urls::*;
pub use sealed_product::*;
pub use set::*;

#[cfg(test)]
mod tests {
    use crate::atomic_cards::AtomicCards;

    #[tokio::test]
    async fn all_printings() {
        let source = std::fs::read("./AtomicCards.json").unwrap();
        let jd = &mut serde_json::Deserializer::from_slice(&source);
        let _: AtomicCards = serde_path_to_error::deserialize(jd).unwrap_or_else(|err| {
            let path = err.path().to_string();
            panic!("{path} -> {err}")
        });
    }
}
