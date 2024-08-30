-- This file should undo anything in `up.sql`

ALTER TABLE "problems" DROP COLUMN "notes";
ALTER TABLE "problems" ADD COLUMN "notes" TEXT NOT NULL;




