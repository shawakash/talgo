-- This file should undo anything in `up.sql`
DROP TRIGGER IF EXISTS update_languages_timestamp ON languages;
DROP FUNCTION IF EXISTS update_languages_timestamp();
DROP TABLE IF EXISTS languages;
