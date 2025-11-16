CREATE SCHEMA IF NOT EXISTS warehouse;

CREATE TABLE IF NOT EXISTS warehouse.rss_items (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    source TEXT NOT NULL,
    title TEXT NOT NULL,
    link TEXT NOT NULL,
    summary TEXT,
    published_at TIMESTAMP WITH TIME ZONE NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_rss_items_source ON warehouse.rss_items (source);
CREATE INDEX IF NOT EXISTS idx_rss_items_published_at ON warehouse.rss_items (published_at);
CREATE INDEX IF NOT EXISTS idx_rss_items_created_at ON warehouse.rss_items (created_at);
