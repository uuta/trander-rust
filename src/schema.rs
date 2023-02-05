// @generated automatically by Diesel CLI.

diesel::table! {
    google_place_ids (id) {
        id -> Unsigned<Bigint>,
        place_id -> Varchar,
        name -> Varchar,
        icon -> Varchar,
        rating -> Nullable<Double>,
        photo -> Nullable<Varchar>,
        vicinity -> Nullable<Varchar>,
        user_ratings_total -> Nullable<Integer>,
        price_level -> Nullable<Integer>,
        lat -> Decimal,
        lng -> Decimal,
        rating_star -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    m_countries (id) {
        id -> Unsigned<Bigint>,
        country_code -> Varchar,
        name -> Varchar,
        exist_in_geo_db_cities -> Bool,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    m_directions (direction_id) {
        direction_id -> Unsigned<Integer>,
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
        prefix -> Varchar,
        exist -> Bool,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    m_ratings (id) {
        id -> Unsigned<Integer>,
        class_name -> Varchar,
        min -> Double,
        max -> Double,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(m_exist_country_prefixes -> m_countries (country_id));

diesel::allow_tables_to_appear_in_same_query!(
    google_place_ids,
    m_countries,
    m_directions,
    m_exist_country_prefixes,
    m_ratings,
);
