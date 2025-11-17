use clap::{Parser, Subcommand};
use dotenvy::dotenv;
use raven_news::db::create_pg_pool;
use raven_news::db::stats::{count_daily_rss_items, count_source_rss_items, count_total_rss_items};
use raven_news::ingest::{fetch_all_and_insert, run_scheduler};

#[derive(Parser)]
#[command(name = "raven-news")]
#[command(about = "RSS ingestion CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Fetch RSS feeds one time and insert into DB (force snapshot)
    FetchOnce,

    /// Run continuous ingestion loop (every 60 seconds)
    Run,

    /// Show ingestion statistics
    Stats {
        #[command(subcommand)]
        category: StatsCategory,
    },
}

#[derive(Subcommand)]
enum StatsCategory {
    Total,
    Daily,
    Source { name: String },
}

// CLI entry point
#[tokio::main]
async fn main() {
    dotenv().ok();
    let cli = Cli::parse();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = create_pg_pool(&database_url).await;

    match cli.command {
        Commands::FetchOnce => {
            fetch_all_and_insert(&pool).await.unwrap();
        }
        Commands::Run => {
            run_scheduler(pool).await;
        }
        Commands::Stats { category } => match category {
            StatsCategory::Total => {
                let cnt = count_total_rss_items(&pool).await.unwrap();
                println!("Total RSS items: {cnt}");
            }
            StatsCategory::Daily => {
                let cnt = count_daily_rss_items(&pool).await.unwrap();
                println!("Daily RSS items: {cnt}");
            }
            StatsCategory::Source { name } => {
                let cnt = count_source_rss_items(&pool, &name).await.unwrap();
                println!("RSS items for {name}: {cnt}");
            }
        },
    };
}
