-- Your SQL goes here
CREATE TABLE "users" (
    "id" UUID NOT NULL PRIMARY KEY DEFAULT gen_random_uuid(),
    "username" VARCHAR(255) NOT NULL,
    "email" VARCHAR(255) NOT NULL,
    "password_hash" VARCHAR(255) NOT NULL,
    "full_name" VARCHAR(255),
    "bio" TEXT NOT NULL DEFAULT '',
    "country" VARCHAR(255),
    "organization" VARCHAR(255),
    "rating" INT4 NOT NULL DEFAULT 0,
    "max_rating" INT4 NOT NULL DEFAULT 0,
    "rank" VARCHAR(50) NOT NULL DEFAULT 'Newbie',
    "contribution" INT4 NOT NULL DEFAULT 0,
    "friend_count" INT4 NOT NULL DEFAULT 0,
    "is_admin" BOOL NOT NULL DEFAULT false,
    "is_banned" BOOL NOT NULL DEFAULT false,
    "last_online" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "registration_time" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "avatar_url" VARCHAR(255),
    "github_username" VARCHAR(255),
    "preferred_language" VARCHAR(50),
    "problems_solved" INT4 NOT NULL DEFAULT 0,
    "contests_participated" INT4 NOT NULL DEFAULT 0,
    "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT unique_username UNIQUE (username),
    CONSTRAINT unique_email UNIQUE (email)
);
