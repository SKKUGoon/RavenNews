use crate::error::RssParseError;
use crate::rss::strip_cdata;
use crate::rss::{RssItem, RssParser, RssResult};
use chrono::{DateTime, Utc};
use quick_xml::Reader;
use quick_xml::events::Event;
use quick_xml::name::QName;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReutersRssItem {
    pub title: String,
    pub link: String,
    pub description: Option<String>,
    pub published_at: DateTime<Utc>,
    pub creator: Option<String>,
}

impl ReutersRssItem {
    pub fn into_rss_item(self) -> RssItem {
        RssItem::new(
            "reuters",
            self.title,
            self.link,
            self.description,
            Some(self.published_at),
        )
    }
}

pub struct ReutersRssParser;

impl RssParser for ReutersRssParser {
    fn parse(&self, xml: &str) -> RssResult<Vec<RssItem>> {
        let mut reader: Reader<&[u8]> = Reader::from_str(xml);

        let mut buf = Vec::new();
        let mut items = Vec::new();
        let mut in_item = false;

        // fields
        let mut title: Option<String> = None;
        let mut link: Option<String> = None;
        let mut description: Option<String> = None;
        let mut creator: Option<String> = None;
        let mut published_at: Option<String> = None;

        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) => match e.name().as_ref() {
                    b"item" => {
                        in_item = true;

                        title = None;
                        link = None;
                        description = None;
                        creator = None;
                        published_at = None;
                    }
                    b"title" if in_item => {
                        title = reader
                            .read_text(QName(b"title"))
                            .ok()
                            .map(|s| strip_cdata(&s));
                    }
                    b"link" if in_item => {
                        link = reader
                            .read_text(QName(b"link"))
                            .ok()
                            .map(|s| s.into_owned());
                    }
                    b"description" if in_item => {
                        description = reader
                            .read_text(QName(b"description"))
                            .ok()
                            .map(|s| strip_cdata(&s));
                    }
                    b"pubDate" if in_item => {
                        published_at = reader
                            .read_text(QName(b"pubDate"))
                            .ok()
                            .map(|s| s.into_owned());
                    }
                    b"creator" if in_item => {
                        creator = reader
                            .read_text(QName(b"creator"))
                            .ok()
                            .map(|s| s.into_owned());
                    }
                    _ => {}
                },

                Ok(Event::End(ref e)) if e.name().as_ref() == b"item" => {
                    in_item = false;

                    if let (Some(title), Some(link), Some(pubdate)) =
                        (title.clone(), link.clone(), published_at.clone())
                    {
                        // parse pubDate into chrono
                        let parsed_dt = DateTime::parse_from_rfc2822(&pubdate)
                            .map(|dt| dt.with_timezone(&Utc))
                            .unwrap_or_else(|_| Utc::now());

                        let item = ReutersRssItem {
                            title,
                            link,
                            description: description.clone(),
                            creator: creator.clone(),
                            published_at: parsed_dt,
                        };

                        items.push(item.into_rss_item());
                    }
                }

                Ok(Event::Eof) => break,

                Err(e) => return Err(RssParseError::Xml(e.to_string())),

                _ => {}
            }
        }

        Ok(items)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::blocking;

    fn fetch_xml(url: &str) -> String {
        blocking::get(url)
            .expect("HTTP GET failed")
            .text()
            .expect("Failed to read response body")
    }

    #[test]
    fn test_reuters_rss_financial() {
        // https://ir.thomsonreuters.com/rss/news-releases.xml?items=15
        let xml = fetch_xml("https://ir.thomsonreuters.com/rss/news-releases.xml?items=15");
        let parser = ReutersRssParser;
        let items = parser.parse(&xml).expect("Failed to parse XML");

        // test print
        println!("Reuters Financial RSS: {} items", items.len());

        // Print first 3 examples
        for item in items.iter().take(3) {
            println!("TITLE: {}", item.title);
            println!("LINK : {}", item.link);
            println!(
                "DESCRIPTION: {}",
                item.summary.as_ref().unwrap_or(&"N/A".to_string())
            );
            println!("------");
        }
    }

    #[test]
    fn test_reuters_rss_events() {
        // https://ir.thomsonreuters.com/rss/events.xml?items=15
        let xml = fetch_xml("https://ir.thomsonreuters.com/rss/events.xml?items=15");
        let parser = ReutersRssParser;
        let items = parser.parse(&xml).expect("Failed to parse XML");

        // test print
        println!("Reuters Events RSS: {} items", items.len());

        // Print first 3 examples
        for item in items.iter().take(3) {
            println!("TITLE: {}", item.title);
            println!("LINK : {}", item.link);
            println!(
                "DESCRIPTION: {}",
                item.summary.as_ref().unwrap_or(&"N/A".to_string())
            );
            println!("------");
        }
    }

    #[test]
    fn test_reuters_rss_secfilings() {
        // https://ir.thomsonreuters.com/rss/sec-filings.xml?items=15
        let xml = fetch_xml("https://ir.thomsonreuters.com/rss/sec-filings.xml?items=15");
        let parser = ReutersRssParser;
        let items = parser.parse(&xml).expect("Failed to parse XML");

        // test print
        println!("Reuters SEC Filings RSS: {} items", items.len());

        // Print first 3 examples
        for item in items.iter().take(3) {
            println!("TITLE: {}", item.title);
            println!("LINK : {}", item.link);
            println!(
                "DESCRIPTION: {}",
                item.summary.as_ref().unwrap_or(&"N/A".to_string())
            );
            println!("------");
        }
    }
}
