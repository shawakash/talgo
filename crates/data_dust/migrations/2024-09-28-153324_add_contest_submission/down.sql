-- This file should undo anything in `up.sql`
DROP TRIGGER IF EXISTS set_contest_sub ON submissions;
DROP FUNCTION IF EXISTS update_contest_sub();
ALTER TABLE submissions DROP COLUMN IF EXISTS contest_sub;
