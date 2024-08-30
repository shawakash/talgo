-- Your SQL goes here


CREATE TABLE "submissions"(
	"id" INT8 NOT NULL PRIMARY KEY,
	"user_id" UUID NOT NULL,
	"problem_id" INT4 NOT NULL,
	"contest_id" INT4,
	"language" VARCHAR(50) NOT NULL,
	"code" TEXT NOT NULL,
	"status" INT2 NOT NULL,
	"execution_time_ms" INT4 NOT NULL,
	"memory_used_kb" INT4 NOT NULL,
	"submitted_at" TIMESTAMP NOT NULL,
	"verdict" TEXT,
	"score" FLOAT4,
	"test_cases_passed" INT4,
	"total_test_cases" INT4
);

