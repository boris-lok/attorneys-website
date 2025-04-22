-- Add down migration script here
-- Drop trigger first (as it depends on the function)
DROP TRIGGER IF EXISTS article_viewed ON article_views;

-- Then drop the function
DROP FUNCTION IF EXISTS update_article_view_counts();

DROP TABLE article_view_counts;
DROP TABLE article_views;
