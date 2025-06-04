use crate::generate::SeriesEntry;

pub enum HomeSectionType {
    SingleRowNormal,
    SingleRowLarge,
    DoubleRow,
    Featured,
}

pub struct HomeSection {
    pub id: String,
    pub title: String,
    pub contain_more_items: bool,
    pub section_type: HomeSectionType,
    pub entries: Vec<SeriesEntry>,
}
