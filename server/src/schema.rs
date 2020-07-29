table! {
    entries (id) {
        id -> Int4,
        user_id -> Int4,
        meal_type_id -> Int4,
        hunger_before -> Int4,
        hunger_after -> Int4,
        timestamp -> Timestamp,
        comments -> Nullable<Varchar>,
    }
}

table! {
    foods (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    meal_types (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    meals (food_id, entry_id) {
        food_id -> Int4,
        entry_id -> Int4,
    }
}

table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        password -> Varchar,
    }
}

joinable!(entries -> meal_types (meal_type_id));
joinable!(entries -> users (user_id));
joinable!(meals -> entries (entry_id));
joinable!(meals -> foods (food_id));

allow_tables_to_appear_in_same_query!(
    entries,
    foods,
    meal_types,
    meals,
    users,
);
