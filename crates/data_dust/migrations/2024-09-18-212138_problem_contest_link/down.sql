-- This file should undo anything in `up.sql`
ALTER TABLE "problems" ALTER COLUMN "contest_id" DROP NOT NULL;
