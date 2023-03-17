// @generated automatically by Diesel CLI.

diesel::table! {
    cost_items (id) {
        id -> Int8,
        name -> Varchar,
        price -> Numeric,
        notes -> Nullable<Text>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    cost_items,
);
