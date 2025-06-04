use crate::{
    base::{home_section::HomeSection, source_info::SourceInfo},
    error::SourceResult,
    generate::SeriesEntry,
};

pub struct SourceInstance {
    pub active: bool,
    pub home_page: fn() -> SourceResult<Vec<HomeSection>>,
    pub search: fn(query: String, filters: Vec<String>) -> SourceResult<Vec<SeriesEntry>>,
    pub metadata: SourceInfo,
}
