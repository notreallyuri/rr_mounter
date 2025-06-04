use std::pin::Pin;

use crate::{
    base::{HomeSection, PaginatedResponse, SearchRequest, SourceInfo, SourceSetting},
    error::SourceResult,
    generate::SeriesEntry,
};

pub struct SourceInstance {
    pub active: bool,
    pub settings: Vec<SourceSetting>,
    pub home_page: fn() -> Pin<Box<dyn Future<Output = SourceResult<Vec<HomeSection>>>>>,
    pub search: fn(
        SearchRequest,
    )
        -> Pin<Box<dyn Future<Output = SourceResult<PaginatedResponse<SeriesEntry>>>>>,
    pub metadata: SourceInfo,
}
