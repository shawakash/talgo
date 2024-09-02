-- This file should undo anything in `up.sql`


ALTER TABLE "submissions" DROP COLUMN "contest_id";
ALTER TABLE "submissions" ADD COLUMN "contest_id" INT4;



