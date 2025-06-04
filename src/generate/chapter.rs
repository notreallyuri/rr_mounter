use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chapter {
    // Unique identifier for the chapter
    pub id: String,
    pub series_id: String,
    pub source_id: String,

    // Basic information about the chapter
    pub title: String,
    pub chapter_number: String,
    pub volume_number: String,
    pub group_name: String,

    // Additional metadata about the chapter
    pub time: u64,
    pub read: bool,
}
