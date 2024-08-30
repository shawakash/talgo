-- Your SQL goes here
CREATE TABLE "users"(
	"id" UUID NOT NULL PRIMARY KEY,
	"username" VARCHAR(255) NOT NULL,
	"email" VARCHAR(255) NOT NULL,
	"password_hash" VARCHAR(255) NOT NULL,
	"full_name" VARCHAR(255),
	"bio" TEXT NOT NULL,
	"country" VARCHAR(255),
	"organization" VARCHAR(255),
	"rating" INT4 NOT NULL,
	"max_rating" INT4 NOT NULL,
	"rank" VARCHAR(50) NOT NULL,
	"contribution" INT4 NOT NULL,
	"friend_count" INT4 NOT NULL,
	"is_admin" BOOL NOT NULL,
	"is_banned" BOOL NOT NULL,
	"last_online" TIMESTAMP NOT NULL,
	"registration_time" TIMESTAMP NOT NULL,
	"avatar_url" VARCHAR(255),
	"github_username" VARCHAR(255),
	"preferred_language" VARCHAR(50),
	"problems_solved" INT4 NOT NULL,
	"contests_participated" INT4 NOT NULL,
	"created_at" TIMESTAMP NOT NULL,
	"updated_at" TIMESTAMP NOT NULL
);

