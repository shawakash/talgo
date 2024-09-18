// @generated automatically by Diesel CLI.

diesel::table! {
    contests (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        description -> Nullable<Text>,
        start_time -> Timestamp,
        end_time -> Timestamp,
        duration_seconds -> Int4,
        #[max_length = 50]
        visibility -> Varchar,
        #[max_length = 50]
        contest_type -> Varchar,
        created_by -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        is_rated -> Bool,
        max_participants -> Nullable<Int4>,
        registration_open -> Bool,
        registration_deadline -> Nullable<Timestamp>,
        #[max_length = 50]
        scoring_system -> Varchar,
        penalty_seconds -> Int4,
        frozen_time_seconds -> Nullable<Int4>,
    }
}

diesel::table! {
    problems (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 50]
        difficulty -> Varchar,
        statement -> Text,
        input_specification -> Text,
        output_specification -> Text,
        time_limit_ms -> Int4,
        memory_limit_kb -> Int4,
        sample_input -> Text,
        sample_output -> Text,
        author_id -> Uuid,
        contest_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        is_public -> Bool,
        points -> Nullable<Int4>,
        solved_count -> Int4,
        attempted_count -> Int4,
        notes -> Nullable<Text>,
    }
}

diesel::table! {
    submissions (id) {
        id -> Int8,
        user_id -> Uuid,
        problem_id -> Int4,
        #[max_length = 50]
        language -> Varchar,
        code -> Text,
        status -> Int2,
        execution_time_ms -> Int4,
        memory_used_kb -> Int4,
        submitted_at -> Timestamp,
        verdict -> Nullable<Text>,
        score -> Nullable<Float4>,
        test_cases_passed -> Nullable<Int4>,
        total_test_cases -> Nullable<Int4>,
        contest_id -> Int4,
    }
}

diesel::table! {
    user_problems (user_id, problem_id, submission_id) {
        user_id -> Uuid,
        problem_id -> Int4,
        submission_id -> Int8,
        solved_at -> Timestamp,
        verdict -> Nullable<Text>,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        #[max_length = 255]
        username -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        password_hash -> Varchar,
        #[max_length = 255]
        full_name -> Nullable<Varchar>,
        #[max_length = 255]
        country -> Nullable<Varchar>,
        #[max_length = 255]
        organization -> Nullable<Varchar>,
        rating -> Int4,
        max_rating -> Int4,
        #[max_length = 50]
        rank -> Varchar,
        contribution -> Int4,
        friend_count -> Int4,
        is_admin -> Bool,
        is_banned -> Bool,
        last_online -> Timestamp,
        registration_time -> Timestamp,
        #[max_length = 255]
        avatar_url -> Nullable<Varchar>,
        #[max_length = 255]
        github_username -> Nullable<Varchar>,
        #[max_length = 50]
        preferred_language -> Nullable<Varchar>,
        problems_solved -> Int4,
        contests_participated -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        #[max_length = 254]
        bio -> Nullable<Varchar>,
    }
}

diesel::joinable!(contests -> users (created_by));
diesel::joinable!(problems -> contests (contest_id));
diesel::joinable!(problems -> users (author_id));
diesel::joinable!(submissions -> problems (problem_id));
diesel::joinable!(submissions -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    contests,
    problems,
    submissions,
    user_problems,
    users,
);
