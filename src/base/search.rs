use crate::base::{ContentRating, LanguageKey};
use crate::generate::Status;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchRequest {
    /// The search query string (manga title, author, etc.)
    pub query: String,

    /// Pagination information
    pub pagination: PaginationRequest,

    /// Structured filters for common manga attributes
    pub filters: SearchFilters,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationRequest {
    /// Page number (1-indexed)
    pub page: u32,

    /// Number of items per page
    pub per_page: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchFilters {
    /// Filter by genres (e.g., "Action", "Romance", "Comedy")
    pub genres: Option<Vec<String>>,

    /// Filter by publication status
    pub status: Option<Status>,

    /// Filter by content rating
    pub content_rating: Option<ContentRating>,

    /// Filter by original language
    pub language: Option<LanguageKey>,

    /// Filter by publication year range (start_year, end_year)
    pub year_range: Option<(u32, u32)>,

    /// Filter by author names
    pub authors: Option<Vec<String>>,

    /// Filter by artist names
    pub artists: Option<Vec<String>>,

    /// Custom filters specific to individual manga sources
    /// Key-value pairs where the key is the filter name and value is the filter value
    /// Examples: {"sort_by": "latest"}, {"has_cover": "true"}, {"min_chapters": "10"}
    pub custom_filters: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    /// The actual items for this page
    pub items: Vec<T>,

    /// Current page number (1-indexed)
    pub current_page: u32,

    /// Total number of pages (if known by the source)
    pub total_pages: Option<u32>,

    /// Whether there's a next page available
    pub has_next_page: bool,

    /// Total number of items (if known by the source)
    pub total_items: Option<u32>,
}

impl Default for PaginationRequest {
    fn default() -> Self {
        Self {
            page: 1,
            per_page: 20,
        }
    }
}

impl Default for SearchFilters {
    fn default() -> Self {
        Self {
            genres: None,
            status: None,
            content_rating: None,
            language: None,
            year_range: None,
            authors: None,
            artists: None,
            custom_filters: HashMap::new(),
        }
    }
}

impl SearchRequest {
    /// Create a simple search request with just a query
    pub fn simple(query: String) -> Self {
        Self {
            query,
            pagination: PaginationRequest::default(),
            filters: SearchFilters::default(),
        }
    }

    /// Create a search request with custom pagination
    pub fn with_pagination(query: String, page: u32, per_page: u32) -> Self {
        Self {
            query,
            pagination: PaginationRequest { page, per_page },
            filters: SearchFilters::default(),
        }
    }

    /// Builder method to add genre filters
    pub fn with_genres(mut self, genres: Vec<String>) -> Self {
        self.filters.genres = Some(genres);
        self
    }

    /// Builder method to add status filter
    pub fn with_status(mut self, status: Status) -> Self {
        self.filters.status = Some(status);
        self
    }

    /// Builder method to add custom filter
    pub fn with_custom_filter(mut self, key: String, value: String) -> Self {
        self.filters.custom_filters.insert(key, value);
        self
    }
}

impl<T> PaginatedResponse<T> {
    /// Create a new paginated response
    pub fn new(items: Vec<T>, current_page: u32, has_next_page: bool) -> Self {
        Self {
            items,
            current_page,
            total_pages: None,
            has_next_page,
            total_items: None,
        }
    }

    /// Create a paginated response with total information
    pub fn with_totals(
        items: Vec<T>,
        current_page: u32,
        total_pages: u32,
        total_items: u32,
    ) -> Self {
        Self {
            items,
            current_page,
            total_pages: Some(total_pages),
            has_next_page: current_page < total_pages,
            total_items: Some(total_items),
        }
    }

    /// Check if this is the first page
    pub fn is_first_page(&self) -> bool {
        self.current_page == 1
    }

    /// Get the number of items in this response
    pub fn item_count(&self) -> usize {
        self.items.len()
    }
}
