// @generated automatically by Diesel CLI.

diesel::table! {
    particles (part_id) {
        part_id -> Int4,
        part_type -> Varchar,
        part_name -> Varchar,
        mass -> Int8,
        charge -> Varchar,
        spin -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
