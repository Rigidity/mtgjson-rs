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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_printings() {
        let _: AllPrintings =
            reqwest::blocking::get("https://mtgjson.com/api/v5/AllPrintings.json")
                .unwrap()
                .json()
                .unwrap();
    }
}
