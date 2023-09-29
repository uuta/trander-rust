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
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    m_countries (id) {
        id -> Unsigned<Bigint>,
        #[max_length = 255]
        country_code -> Varchar,
        #[max_length = 255]
        name -> Varchar,
        exist_in_geo_db_cities -> Bool,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    m_directions (direction_id) {
        direction_id -> Unsigned<Integer>,
        #[max_length = 255]
        direction_name -> Varchar,
        min_angle -> Double,
        max_angle -> Double,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    m_exist_country_prefixes (id) {
        id -> Unsigned<Bigint>,
        country_id -> Unsigned<Bigint>,
        #[max_length = 255]
        prefix -> Varchar,
        exist -> Bool,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    m_ratings (id) {
        id -> Unsigned<Integer>,
        #[max_length = 255]
        class_name -> Varchar,
        min -> Double,
        max -> Double,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    m_ways (id) {
        id -> Unsigned<Integer>,
        way_id -> Integer,
        recommend_frequency -> Integer,
        min_distance -> Integer,
        max_distance -> Integer,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    request_count_historys (id) {
        id -> Unsigned<Bigint>,
        user_id -> Unsigned<Bigint>,
        type_id -> Integer,
        created_at -> Nullable<Timestamp>,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    request_limits (id) {
        id -> Unsigned<Integer>,
        user_id -> Unsigned<Bigint>,
        request_limit -> Unsigned<Bigint>,
        first_requested_at -> Datetime,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    setting_historys (id) {
        id -> Unsigned<Bigint>,
        setting_id -> Unsigned<Bigint>,
        min_distance -> Integer,
        max_distance -> Integer,
        direction_type -> Smallint,
        created_at -> Nullable<Timestamp>,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    settings (id) {
        id -> Unsigned<Bigint>,
        user_id -> Unsigned<Bigint>,
        min_distance -> Integer,
        max_distance -> Integer,
        direction_type -> Smallint,
        created_at -> Nullable<Timestamp>,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
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
        #[max_length = 255]
        avatar -> Nullable<Varchar>,
        #[max_length = 100]
        remember_token -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        check_registration -> Bool,
    }
}

diesel::joinable!(m_exist_country_prefixes -> m_countries (country_id));
diesel::joinable!(setting_historys -> settings (setting_id));
diesel::joinable!(settings -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    google_place_ids,
    m_countries,
    m_directions,
    m_exist_country_prefixes,
    m_ratings,
    m_ways,
    request_count_historys,
    request_limits,
    setting_historys,
    settings,
    users,
);
