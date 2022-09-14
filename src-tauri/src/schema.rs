diesel::table! {
    scripts (id) {
        id -> Int4,
        name -> Varchar,
        description -> Nullable<Text>,
    }
}

diesel::table! {
    symbols (id) {
        id -> Int4,
        unicode_number -> Nullable<Varchar>,
        identifier -> Varchar,
        is_composite -> Bool,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    scripts,
    symbols,
);
