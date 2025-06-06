use std::pin::Pin;

use crate::{
    base::{HomeSection, PaginatedResponse, SearchRequest},
    error::SourceResult,
    generate::{Series, SeriesEntry},
};

pub struct SourceInstance {
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
}
