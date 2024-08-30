-- Your SQL goes here




CREATE TABLE "user_problems"(
	"user_id" UUID NOT NULL,
	"problem_id" INT4 NOT NULL,
	"submission_id" INT8 NOT NULL,
	"solved_at" TIMESTAMP NOT NULL,
	"verdict" TEXT,
	PRIMARY KEY("user_id", "problem_id", "submission_id")
);

