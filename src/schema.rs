// @generated automatically by Diesel CLI.

diesel::table! {
    education (id) {
        id -> Integer,
        institution -> Text,
        degree -> Text,
        field_of_study -> Nullable<Text>,
        start_date -> Date,
        end_date -> Nullable<Date>,
        gpa -> Nullable<Text>,
        description -> Nullable<Text>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    personal_info (id) {
        id -> Integer,
        name -> Text,
        title -> Text,
        email -> Text,
        phone -> Nullable<Text>,
        location -> Nullable<Text>,
        summary -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    projects (id) {
        id -> Integer,
        name -> Text,
        description -> Text,
        technologies -> Nullable<Text>,
        start_date -> Nullable<Date>,
        end_date -> Nullable<Date>,
        url -> Nullable<Text>,
        github_url -> Nullable<Text>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    skills (id) {
        id -> Integer,
        name -> Text,
        category -> Text,
        proficiency -> Integer,
        created_at -> Timestamp,
    }
}

diesel::table! {
    work_experiences (id) {
        id -> Integer,
        company -> Text,
        position -> Text,
        start_date -> Date,
        end_date -> Nullable<Date>,
        description -> Nullable<Text>,
        achievements -> Nullable<Text>,
        created_at -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    education,
    personal_info,
    projects,
    skills,
    work_experiences,
);
