-- Your SQL goes here




ALTER TABLE "users" DROP COLUMN "bio";
ALTER TABLE "users" ADD COLUMN "bio" VARCHAR(254);

