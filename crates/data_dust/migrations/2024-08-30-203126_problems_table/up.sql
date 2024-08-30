-- Your SQL goes here

CREATE TABLE "problems"(
	"id" INT4 NOT NULL PRIMARY KEY,
	"name" VARCHAR(255) NOT NULL,
	"difficulty" VARCHAR(50) NOT NULL,
	"statement" TEXT NOT NULL,
	"input_specification" TEXT NOT NULL,
	"output_specification" TEXT NOT NULL,
	"time_limit_ms" INT4 NOT NULL,
	"memory_limit_kb" INT4 NOT NULL,
	"sample_input" TEXT NOT NULL,
	"sample_output" TEXT NOT NULL,
	"notes" TEXT NOT NULL,
	"author_id" UUID NOT NULL,
	"contest_id" INT4,
	"created_at" TIMESTAMP NOT NULL,
	"updated_at" TIMESTAMP NOT NULL,
	"is_public" BOOL NOT NULL,
	"points" INT4,
	"solved_count" INT4 NOT NULL,
	"attempted_count" INT4 NOT NULL
);

