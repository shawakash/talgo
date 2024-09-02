-- Your SQL goes here

CREATE TABLE "problems" (
    "id" SERIAL PRIMARY KEY,
    "name" VARCHAR(255) NOT NULL,
    "difficulty" VARCHAR(50) NOT NULL DEFAULT 'Medium',
    "statement" TEXT NOT NULL,
    "input_specification" TEXT NOT NULL,
    "output_specification" TEXT NOT NULL,
    "time_limit_ms" INT4 NOT NULL DEFAULT 1000,
    "memory_limit_kb" INT4 NOT NULL DEFAULT 256000,
    "sample_input" TEXT NOT NULL,
    "sample_output" TEXT NOT NULL,
    "notes" TEXT NOT NULL DEFAULT '',
    "author_id" UUID NOT NULL,
    "contest_id" INT4,
    "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "is_public" BOOL NOT NULL DEFAULT false,
    "points" INT4 DEFAULT 100,
    "solved_count" INT4 NOT NULL DEFAULT 0,
    "attempted_count" INT4 NOT NULL DEFAULT 0,
    CONSTRAINT fk_author FOREIGN KEY(author_id) REFERENCES users(id),
    CONSTRAINT fk_contest FOREIGN KEY(contest_id) REFERENCES contests(id)
);
