use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{meta::Meta, set::Set};

/// Every printing of every card, grouped by set.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AllPrintings {
    /// Metadata about the data set.
    pub meta: Meta,

    /// Sets grouped by name.
    pub data: HashMap<String, Set>,
}
