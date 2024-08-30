-- This file should undo anything in `up.sql`
ALTER TABLE "users" DROP COLUMN "bio";
ALTER TABLE "users" ADD COLUMN "bio" TEXT NOT NULL;

