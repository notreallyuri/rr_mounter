use bitflags::bitflags;
use serde::{Deserialize, Serialize};

bitflags! {
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct SourceIntents: u32 {
        const MANGA_CHAPTERS = 1<<0;
        const MANGA_TRACKING = 1<<1;
        const HOMEPAGE_SECTIONS = 1<<2;
        const CLOADFARE_BYPASS_REQUIRED = 1<<3;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ContentRating {
    Everyone,
    Mature,
    Adult,
    Multi,
}
