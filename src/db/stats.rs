use sqlx::PgPool;

pub async fn count_total_rss_items(pool: &PgPool) -> Result<i64, sqlx::Error> {
    let count = sqlx::query_scalar!(
        r#"
        SELECT COUNT(*) FROM warehouse.rss_items
        "#
    )
    .fetch_one(pool)
    .await?;

    Ok(count.unwrap_or(0))
}

pub async fn count_daily_rss_items(pool: &PgPool) -> Result<i64, sqlx::Error> {
    let count = sqlx::query_scalar!(
        r#"
        SELECT COUNT(*) FROM warehouse.rss_items
        WHERE published_at >= DATE_TRUNC('day', NOW())
        "#
    )
    .fetch_one(pool)
    .await?;

    Ok(count.unwrap_or(0))
}

pub async fn count_source_rss_items(pool: &PgPool, source: &str) -> Result<i64, sqlx::Error> {
    let count = sqlx::query_scalar!(
        r#"
        SELECT COUNT(*) FROM warehouse.rss_items
        WHERE source = $1
        "#,
        source
    )
    .fetch_one(pool)
    .await?;

    Ok(count.unwrap_or(0))
}
