![Rust Edition](https://img.shields.io/badge/Rust-Edition%202025-b7410e) ![PostgreSQL](https://img.shields.io/badge/PostgreSQL-17%2B-336791) ![SQLx](https://img.shields.io/badge/sqlx-0.8-blue)

# Raven News

Raven News is a Rust-based toolkit for ingesting, normalizing, and storing RSS feeds from financial news providers such as Reuters, Bloomberg, and CoinDesk. The project focuses on reliable parsing, source-aware de-duplication, and an opinionated PostgreSQL schema for long-term warehousing.

## Highlights

- Source-specific RSS parsers built with `quick-xml`, each conforming to a shared `RssParser` trait.
- Stable `RssItem` identifiers generated with SHA-256 fingerprints to avoid duplicates across runs.
- PostgreSQL-backed persistence layer powered by `sqlx`, ready for warehousing and downstream analytics.
- Async-ready foundation using `tokio`, with integration tests that exercise real feeds.

## Prerequisites

- Rust toolchain (`rustup` recommended).
- `cargo` (bundled with Rust).
- `sqlx-cli` for applying database migrations.
- PostgreSQL instance (local or remote).

## Getting Started

1. Install the Rust toolchain:
   ```bash
   rustup toolchain install stable
   ```
2. Install the SQLx CLI:
   ```bash
   cargo install sqlx-cli
   ```
3. Create a PostgreSQL database and export the connection string:
   ```bash
   export DATABASE_URL=postgres://postgres:password@localhost:5432/raven_news
   ```
4. Apply the migrations:
   ```bash
   sqlx migrate run
   ```
5. (Optional) Create a `.env` file to persist environment variables:
   ```bash
   cat <<'EOF' > .env
   DATABASE_URL=postgres://postgres:password@localhost:5432/raven_news
   EOF
   ```

## Running the Parsers

The project is organized as a reusable library. You can experiment from a `cargo` REPL (e.g., `cargo rustc -- --cfg REPL`) or by creating a small binary in `src/main.rs` that calls into the parsers. Example snippet:

```rust
use rss::reuters::ReutersRssParser;
use rss::RssParser;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let xml = reqwest::get("https://ir.thomsonreuters.com/rss/news-releases.xml?items=5")
        .await?
        .text()
        .await?;
    let parser = ReutersRssParser;
    let items = parser.parse(&xml)?;
    println!("Fetched {} items", items.len());
    Ok(())
}
```

## Tests

- Unit and integration tests can be run with:
  ```bash
  cargo test
  ```
- Database-aware tests expect `DATABASE_URL` to be set (use `.env` or environment variables when invoking `cargo test`).

## Project Structure

```
├── migrations/                # SQLx migrations defining the warehouse schema
├── src/
│   ├── db/                    # PostgreSQL pool factory
│   ├── rss/
│   │   ├── bloomberg.rs       # Bloomberg RSS parser
│   │   ├── coindesk.rs        # CoinDesk RSS parser
│   │   └── reuters.rs         # Reuters RSS parser
│   └── main.rs                # Binary entry point (customize for your needs)
└── Cargo.toml
```

## Next Steps

- Add schedulers or jobs to poll feeds and persist results via the `db` module.
- Extend the schema and parsers to handle additional publishers or enrichments.
- Integrate alerting, search, or downstream analytics pipelines once ingestion is stable.