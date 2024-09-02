-- Your SQL goes here


ALTER TABLE "submissions" DROP COLUMN "contest_id";
ALTER TABLE "submissions" ADD COLUMN "contest_id" INT4 NOT NULL;



