use crate::{ParseContentType, SourceError, SourceResult};
use scraper::Selector;
use std::collections::HashMap;

pub fn create_selectors(selectors: &[(&str, &str)]) -> SourceResult<HashMap<String, Selector>> {
    selectors
        .iter()
        .map(|(name, css)| {
            Selector::parse(css)
                .map(|sel| (name.to_string(), sel))
                .map_err(|e| SourceError::ParseError {
                    details: format!("Failed to parse selector '{}': {}", name, e),
                    content_type: ParseContentType::Html,
                    structure_changed: false,
                })
        })
        .collect()
}
