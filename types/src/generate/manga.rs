use bitflags::bitflags;

pub struct MangaInfo {
    pub cover_url: String,
    pub artist: String,
    pub author: String,
    pub description: String,
    pub status: Status,
    pub hentai: bool,
    pub tags: Vec<Tag>,
    pub titles: Vec<String>,
    pub banner: Option<String>,
    pub rating: Option<u8>,
    pub lang_flag: String,
    pub lang_name: String,
}

pub enum Status {
    Ongoing,
    Completed,
    Hiatus,
    Cancelled,
}

pub struct Tag {
    pub id: String,
    pub label: String,
}

pub struct TagSection {
    pub id: String,
    pub label: String,
    pub tags: Vec<Tag>,
}
