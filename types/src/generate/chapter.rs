use chrono::NaiveDateTime;

pub struct Chapter {
    pub id: String,
    pub chap_num: u32,
    pub name: String,
    pub volume: Option<u32>,
    pub group: String,
    pub time: NaiveDateTime,
    pub sort_index: u32,
}

pub struct ChapterDetails {
    id: String,
    manga_id: String,
    pages: Vec<String>,
}
