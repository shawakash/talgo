-- Your SQL goes here



CREATE TABLE "contests"(
	"id" INT4 NOT NULL PRIMARY KEY,
	"name" VARCHAR(255) NOT NULL,
	"description" TEXT,
	"start_time" TIMESTAMP NOT NULL,
	"end_time" TIMESTAMP NOT NULL,
	"duration_seconds" INT4 NOT NULL,
	"visibility" VARCHAR(50) NOT NULL,
	"contest_type" VARCHAR(50) NOT NULL,
	"created_by" UUID NOT NULL,
	"created_at" TIMESTAMP NOT NULL,
	"updated_at" TIMESTAMP NOT NULL,
	"is_rated" BOOL NOT NULL,
	"max_participants" INT4,
	"registration_open" BOOL NOT NULL,
	"registration_deadline" TIMESTAMP,
	"scoring_system" VARCHAR(50) NOT NULL,
	"penalty_seconds" INT4 NOT NULL,
	"frozen_time_seconds" INT4
);

