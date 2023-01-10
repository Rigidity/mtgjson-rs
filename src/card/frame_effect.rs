use serde::{Deserialize, Serialize};

/// The frame effect of a card.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[allow(missing_docs)]
pub enum FrameEffect {
    ColorShifted,
    Companion,
    CompassLandDfc,
    ConvertDfc,
    Devoid,
    Draft,
    Etched,
    ExtendedArt,
    FanDfc,
    FullArt,
    Gilded,
    Inverted,
    Legendary,
    Lesson,
    Miracle,
    MoonEldraziDfc,
    NyxTouched,
    OriginPwDfc,
    ShatteredGlass,
    Showcase,
    Snow,
    SunMoonDfc,
    Textless,
    Tombstone,
    UpsideDownDfc,
    WaxingAndWaningMoonDfc,
}
