// @generated automatically by Diesel CLI.

diesel::table! {
    google_place_ids (id) {
        id -> Unsigned<Bigint>,
        #[max_length = 255]
        place_id -> Varchar,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        icon -> Varchar,
        rating -> Nullable<Double>,
        #[max_length = 255]
        photo -> Nullable<Varchar>,
        #[max_length = 255]
        vicinity -> Nullable<Varchar>,
        user_ratings_total -> Nullable<Integer>,
        price_level -> Nullable<Integer>,
        lat -> Decimal,
        lng -> Decimal,
        #[max_length = 255]
        rating_star -> Nullable<Varchar>,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

diesel::table! {
    m_ratings (id) {
        id -> Unsigned<Integer>,
        #[max_length = 255]
        class_name -> Varchar,
        min -> Double,
        max -> Double,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

diesel::table! {
    request_count_historys (id) {
        id -> Unsigned<Bigint>,
        user_id -> Unsigned<Bigint>,
        type_id -> Integer,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

diesel::table! {
    request_limits (id) {
        id -> Unsigned<Integer>,
        user_id -> Unsigned<Bigint>,
        request_limit -> Unsigned<Bigint>,
        first_requested_at -> Datetime,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

diesel::table! {
    setting_historys (id) {
        id -> Unsigned<Bigint>,
        setting_id -> Unsigned<Bigint>,
        min_distance -> Integer,
        max_distance -> Integer,
        direction_type -> Smallint,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

diesel::table! {
    settings (id) {
        id -> Unsigned<Bigint>,
        user_id -> Unsigned<Bigint>,
        min_distance -> Integer,
        max_distance -> Integer,
        direction_type -> Smallint,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

diesel::table! {
    users (id) {
        id -> Unsigned<Bigint>,
        #[max_length = 255]
        unique_id -> Nullable<Varchar>,
        name -> Nullable<Text>,
        #[max_length = 255]
        email -> Nullable<Varchar>,
        email_verified_at -> Nullable<Timestamp>,
        #[max_length = 255]
        password -> Nullable<Varchar>,
        #[max_length = 100]
        remember_token -> Nullable<Varchar>,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

diesel::joinable!(setting_historys -> settings (setting_id));
diesel::joinable!(settings -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    google_place_ids,
    m_ratings,
    request_count_historys,
    request_limits,
    setting_historys,
    settings,
    users,
);
