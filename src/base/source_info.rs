use super::badge::Badge;
use super::lang::Language;
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
pub enum ContentRating {
    Everyone,
    Mature,
    Adult,
    Multi,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceInfo {
    // Fields about the extension itself, required for the extension to be recognized and for some features to work
    // Version is read at launch time to check if the extension is up to date
    pub version: String,
    pub name: String,

    // The content rating of the extension, used to filter out extensions based on user preferences
    // By default, users will only see extensions with a content rating of "Everyone"
    // For Mature and Adult extensions, users will need to enable the "Mature Content" setting in the app
    pub content_rating: ContentRating,
    pub description: String,
    pub icon_url: String,

    // Fields about the author of the extension
    pub publisher: String,
    pub publisher_url: Option<String>,

    // Required field for the extension to work
    pub homepage_url: String,

    // Optional fields
    // Metadata about the extension, which is rendered in the extension details page
    pub intents: Option<Vec<SourceIntents>>,
    pub language: Language,
    pub badges: Option<Vec<Badge>>,
}
