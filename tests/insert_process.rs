use dotenvy::dotenv;
use raven_news::db::{create_pg_pool, insert_rss_item};
use raven_news::rss::RssParser;
use raven_news::rss::bloomberg::BloombergRssParser;
use std::fs;

// Integration test
#[tokio::test]
async fn test_bloomberg_rss_insert_integration() {
    dotenv().ok();

    let xml_path = "tests/data/bloomberg_test.xml";
    let xml = fs::read_to_string(xml_path).expect("Failed to read XML file");

    // Setup parser
    let parser = BloombergRssParser;
    let items = parser.parse(&xml).expect("Failed to parse XML");

    // Setup database connection
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = create_pg_pool(&database_url).await;

    // Insert items one-by-one
    for item in &items {
        let inserted = insert_rss_item(&pool, item)
            .await
            .expect("Failed to insert RSS item");

        println!("{} | Inserted {}", item.title, inserted);
    }

    // Test duplicate handling for the first time
    let first = &items[0];
    let duplicate_result = insert_rss_item(&pool, first)
        .await
        .expect("Duplicate check failed");

    assert!(!duplicate_result, "Duplicate should not be re-inserted");
}
