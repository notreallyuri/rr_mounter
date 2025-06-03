pub struct Tag {
    pub id: String,
    pub label: String,
}

pub struct TagSection {
    pub id: String,
    pub label: String,
    pub tags: Vec<Tag>,
}
