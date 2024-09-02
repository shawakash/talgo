-- Your SQL goes here

CREATE TABLE "contests" (
    "id" SERIAL PRIMARY KEY,
    "name" VARCHAR(255) NOT NULL,
    "description" TEXT,
    "start_time" TIMESTAMP NOT NULL,
    "end_time" TIMESTAMP NOT NULL,
    "duration_seconds" INT4 NOT NULL,
    "visibility" VARCHAR(50) NOT NULL DEFAULT 'Public',
    "contest_type" VARCHAR(50) NOT NULL DEFAULT 'Standard',
    "created_by" UUID NOT NULL,
    "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "is_rated" BOOL NOT NULL DEFAULT true,
    "max_participants" INT4,
    "registration_open" BOOL NOT NULL DEFAULT true,
    "registration_deadline" TIMESTAMP,
    "scoring_system" VARCHAR(50) NOT NULL DEFAULT 'Standard',
    "penalty_seconds" INT4 NOT NULL DEFAULT 1200,
    "frozen_time_seconds" INT4,
    CONSTRAINT fk_creator FOREIGN KEY(created_by) REFERENCES users(id),
    CONSTRAINT check_contest_dates CHECK (end_time > start_time),
    CONSTRAINT check_registration_deadline CHECK (registration_deadline IS NULL OR registration_deadline <= start_time)
);
