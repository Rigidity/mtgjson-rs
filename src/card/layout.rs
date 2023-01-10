use serde::{Deserialize, Serialize};

/// The layout of a card.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[allow(missing_docs)]
pub enum Layout {
    Adventure,
    Aftermath,
    ArtSeries,
    Augment,
    Class,
    DoubleFacedToken,
    Emblem,
    Flip,
    Host,
    Leveler,
    Meld,
    ModalDfc,
    Normal,
    Planar,
    ReversibleCard,
    Saga,
    Scheme,
    Split,
    Token,
    Transform,
    Vanguard,
}
