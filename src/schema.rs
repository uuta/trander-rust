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
