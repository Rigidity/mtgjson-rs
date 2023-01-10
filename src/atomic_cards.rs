use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{card::AtomicCard, meta::Meta};

/// Every unique card, grouped by name.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AtomicCards {
    /// Metadata about the data set.
    pub meta: Meta,

    /// Cards grouped by name.
    pub data: HashMap<String, Vec<AtomicCard>>,
}
