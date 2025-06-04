use std::pin::Pin;

use crate::{
    base::{HomeSection, PaginatedResponse, SearchRequest, SourceInfo, SourceSetting},
    error::SourceResult,
    generate::{Series, SeriesEntry},
};

pub struct SourceInstance {
    pub active: bool,
    pub settings: Vec<SourceSetting>,
    pub home_page: fn() -> Pin<Box<dyn Future<Output = SourceResult<Vec<HomeSection>>>>>,
    pub search: fn(
        SearchRequest,
    )
        -> Pin<Box<dyn Future<Output = SourceResult<PaginatedResponse<SeriesEntry>>>>>,
    pub series: fn(id: String) -> Pin<Box<dyn Future<Output = SourceResult<Series>>>>,
    pub chapter: fn(
        series_id: Option<String>,
        chapter_id: String,
    ) -> Pin<Box<dyn Future<Output = SourceResult<Vec<String>>>>>,
    pub metadata: SourceInfo,
}
