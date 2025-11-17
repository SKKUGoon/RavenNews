use crate::db::insert_rss_item;
use crate::rss::{
    RssParser, bloomberg::BloombergRssParser, coindesk::CoindeskRssParser,
    reuters::ReutersRssParser,
};
use sqlx::PgPool;
use tokio::time::{Duration, interval};

async fn fetch_bloomberg_wealth(pool: &PgPool) -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://feeds.bloomberg.com/wealth/news.rss";
    let xml = reqwest::get(url).await?.text().await?;

    let parser = BloombergRssParser;
    let items = parser.parse(&xml)?;

    for item in &items {
        insert_rss_item(pool, item).await?;
    }

    Ok(())
}

async fn fetch_bloomberg_economics(pool: &PgPool) -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://feeds.bloomberg.com/economics/news.rss";
    let xml = reqwest::get(url).await?.text().await?;

    let parser = BloombergRssParser;
    let items = parser.parse(&xml)?;

    for item in &items {
        insert_rss_item(pool, item).await?;
    }

    Ok(())
}

async fn fetch_bloomberg_markets(pool: &PgPool) -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://feeds.bloomberg.com/markets/news.rss";
    let xml = reqwest::get(url).await?.text().await?;

    let parser = BloombergRssParser;
    let items = parser.parse(&xml)?;

    for item in &items {
        insert_rss_item(pool, item).await?;
    }

    Ok(())
}

async fn fetch_coindesk(pool: &PgPool) -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://www.coindesk/com/arc/outboundfeeds/rss";
    let xml = reqwest::get(url).await?.text().await?;

    let parser = CoindeskRssParser;
    let items = parser.parse(&xml)?;

    for item in &items {
        insert_rss_item(pool, item).await?;
    }

    Ok(())
}

async fn fetch_reuters_financial(pool: &PgPool) -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://ir.thomsonreuters.com/rss/news-releases.xml?items=15";
    let xml = reqwest::get(url).await?.text().await?;

    let parser = ReutersRssParser;
    let items = parser.parse(&xml)?;

    for item in &items {
        insert_rss_item(pool, item).await?;
    }

    Ok(())
}

async fn fetch_reuters_events(pool: &PgPool) -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://ir.thomsonreuters.com/rss/events.xml?items=15";
    let xml = reqwest::get(url).await?.text().await?;

    let parser = ReutersRssParser;
    let items = parser.parse(&xml)?;

    for item in &items {
        insert_rss_item(pool, item).await?;
    }

    Ok(())
}

async fn fetch_reuters_secfilings(pool: &PgPool) -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://ir.thomsonreuters.com/rss/sec-filings.xml?items=15";
    let xml = reqwest::get(url).await?.text().await?;

    let parser = ReutersRssParser;
    let items = parser.parse(&xml)?;

    for item in &items {
        insert_rss_item(pool, item).await?;
    }

    Ok(())
}

pub async fn fetch_all_and_insert(pool: &PgPool) -> Result<(), Box<dyn std::error::Error>> {
    fetch_bloomberg_wealth(pool).await?;
    fetch_bloomberg_economics(pool).await?;
    fetch_bloomberg_markets(pool).await?;
    fetch_coindesk(pool).await?;
    fetch_reuters_financial(pool).await?;
    fetch_reuters_events(pool).await?;
    fetch_reuters_secfilings(pool).await?;

    Ok(())
}

pub async fn run_scheduler(pool: PgPool) {
    let mut ticker = interval(Duration::from_secs(60));

    loop {
        ticker.tick().await;

        println!("Running scheduled RSS fetch...");
        if let Err(e) = fetch_all_and_insert(&pool).await {
            eprintln!("Error fetching RSS: {e}");
        }
    }
}
