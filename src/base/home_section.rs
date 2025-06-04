use serde::{Deserialize, Serialize};

use crate::generate::SeriesEntry;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HomeSectionType {
    SingleRowNormal,
    SingleRowLarge,
    DoubleRow,
    Featured,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HomeSection {
    pub id: String,
    pub title: String,
    pub contain_more_items: bool,
    pub section_type: HomeSectionType,
    pub entries: Vec<SeriesEntry>,
}
