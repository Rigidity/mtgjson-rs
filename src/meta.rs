use chrono::NaiveDate;
use semver::Version;
use serde::{Deserialize, Serialize};

/// Describes MTGJSON metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Meta {
    /// The current release date in ISO 8601 format for the MTGJSON build.
    pub date: NaiveDate,

    /// The current SemVer version for the MTGJSON build appended with the build date.
    pub version: Version,
}
