use crate::base::LanguageKey;

pub struct SeriesEntry {
    pub source_id: String,
    pub series_id: String,
    pub title: String,
    pub cover_url: String,
}

pub struct Series {
    // Unique identifier for the manga
    pub source_id: String,
    pub series_id: String,

    // Basic information about the manga
    pub title: String,
    pub alt_titles: Vec<String>,
    pub description: String,
    pub status: Status,
    pub cover_url: String,

    // Additional metadata about the manga
    pub author: Vec<String>,
    pub artist: Vec<String>,
    pub tags: Vec<String>,

    // Information for important filtering
    pub hentai: bool,
    pub original_language: LanguageKey,

    // Statistics about the manga
    pub number_unread: u64,
    pub number_chapters: u64,
}

pub enum Status {
    Ongoing,
    Completed,
    Hiatus,
    Cancelled,
}
