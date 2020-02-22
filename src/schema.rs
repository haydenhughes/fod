table! {
    items (id) {
        id -> Int4,
        name -> Varchar,
        qty -> Int4,
        req_qty -> Nullable<Int4>,
        price -> Nullable<Float4>,
        notes -> Nullable<Text>,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        password -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    items,
    users,
);
