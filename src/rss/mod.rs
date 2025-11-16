pub mod bloomberg;
pub mod coindesk;
pub mod reuters;
// pub mod seeking_alpha;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::error::Error;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct RssItem {
    pub id: Uuid,
    pub source: String,
    pub title: String,
    pub link: String,
    pub summary: Option<String>,
    pub published_at: DateTime<Utc>,
}

impl RssItem {
    pub fn new(
        feed_name: impl Into<String>,
        title: impl Into<String>,
        link: impl Into<String>,
        summary: Option<String>,
        published_at: Option<DateTime<Utc>>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            source: feed_name.into(),
            title: title.into(),
            link: link.into(),
            summary,
            published_at: published_at.unwrap_or_else(Utc::now),
        }
    }
}

pub type RssResult<T> = Result<T, Box<dyn Error>>;

/// Trait: every RSS feed should implement this trait
pub trait RssParser {
    fn parse(&self, xml: &str) -> RssResult<Vec<RssItem>>;
}

// Utility functions
pub fn strip_cdata(text: &str) -> String {
    let t = text.trim();

    if t.starts_with("<![CDATA[") && t.ends_with("]]>") {
        t.trim_start_matches("<![CDATA[")
            .trim_end_matches("]]>")
            .to_string()
    } else {
        text.to_string()
    }
}
