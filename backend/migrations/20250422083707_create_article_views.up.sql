-- Add up migration script here
-- Create article_views table to track individual view events
CREATE TABLE article_views
(
    id         uuid        NOT NULL DEFAULT gen_random_uuid(),
    article_id varchar(32) NOT NULL,
    viewer_ip  inet        NOT NULL,
    user_agent text,
    viewed_at  timestamptz NOT NULL DEFAULT NOW(),
    PRIMARY KEY (id)
);

-- Create article_view_counts table for aggregated statistics
CREATE TABLE article_view_counts
(
    article_id   varchar(32) NOT NULL,
    total_views  bigint      NOT NULL DEFAULT 0,
    unique_views bigint      NOT NULL DEFAULT 0,
    updated_at   timestamptz NOT NULL DEFAULT NOW(),
    PRIMARY KEY (article_id),
    FOREIGN KEY (article_id) REFERENCES resource (id)
);

-- Create trigger to update article_view_counts automatically
CREATE
OR REPLACE FUNCTION update_article_view_counts()
RETURNS TRIGGER AS $$
BEGIN
INSERT INTO article_view_counts (article_id, total_views, unique_views)
VALUES (NEW.article_id, 1, 1) ON CONFLICT (article_id)
    DO
UPDATE SET
    total_views = article_view_counts.total_views + 1,
    unique_views = (
    SELECT COUNT (DISTINCT viewer_ip)
    FROM article_views
    WHERE article_id = NEW.article_id
    ),
    updated_at = NOW();
RETURN NEW;
END;
$$
LANGUAGE plpgsql;

CREATE TRIGGER article_viewed
    AFTER INSERT
    ON article_views
    FOR EACH ROW
    EXECUTE FUNCTION update_article_view_counts();
