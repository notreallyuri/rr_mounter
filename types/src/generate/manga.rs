pub struct MangaInfo {
    pub cover_url: String,
    pub artist: String,
    pub author: String,
    pub description: String,
    pub status: String,
    pub hentai: bool,
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
    Unknown,
}
