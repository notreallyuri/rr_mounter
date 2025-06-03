use rrtypes::base::{ContentRating, Language, LanguageKey, SourceInfo, SourceIntents};

pub fn mangadex_factory() -> SourceInfo {
    SourceInfo {
        version: "0.1.0".to_string(),
        name: "MangaDex".to_string(),
        description: "MangaDex is a manga reader and community platform that allows users to read and share manga.".to_string(),
        content_rating: ContentRating::Multi,
        icon_url: "https://mangadex.org/favicon.ico".to_string(),
        publisher: "Notreallyuri".to_string(),
        publisher_url: Some("".to_string()),
        homepage_url: "https://mangadex.org".to_string(),
        intents: Some(SourceIntents::MANGA_CHAPTERS |  SourceIntents::HOMEPAGE_SECTIONS),
        language: Language::new(LanguageKey::Multi),
        badges: None
    }
}


