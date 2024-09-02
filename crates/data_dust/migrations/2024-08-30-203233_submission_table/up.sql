-- Your SQL goes here


CREATE TABLE submissions (
    "id" BIGSERIAL PRIMARY KEY,
    "user_id" UUID NOT NULL,
    "problem_id" INTEGER NOT NULL,
    "language" VARCHAR(50) NOT NULL,
    "code" TEXT NOT NULL,
    "status" SMALLINT NOT NULL DEFAULT 0,
    "execution_time_ms" INTEGER NOT NULL DEFAULT 0,
    "memory_used_kb" INTEGER NOT NULL DEFAULT 0,
    "submitted_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "verdict" TEXT DEFAULT 'Pending',
    "score" REAL DEFAULT 0,
    "test_cases_passed" INTEGER DEFAULT 0,
    "total_test_cases" INTEGER DEFAULT 0,
    "contest_id" INTEGER NOT NULL,
    CONSTRAINT fk_user FOREIGN KEY (user_id) REFERENCES users(id),
    CONSTRAINT fk_problem FOREIGN KEY (problem_id) REFERENCES problems(id),
    CONSTRAINT fk_contest FOREIGN KEY (contest_id) REFERENCES contests(id)
);
