table! {
    exercise_entries (id) {
        id -> Int4,
        meta_entry_id -> Int4,
        exercise_type_id -> Int4,
        duration -> Timestamp,
    }
}

table! {
    exercise_types (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    foods (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    meal_entries (id) {
        id -> Int4,
        meta_entry_id -> Int4,
        meal_type_id -> Int4,
    }
}

table! {
    meal_types (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    meals (food_id, meal_entry_id) {
        food_id -> Int4,
        meal_entry_id -> Int4,
    }
}

table! {
    meta_entries (id) {
        id -> Int4,
        user_id -> Int4,
        timestamp -> Timestamp,
        comments -> Nullable<Varchar>,
    }
}

table! {
    sleep_entries (id) {
        id -> Int4,
        meta_entry_id -> Int4,
        duration -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        password -> Varchar,
    }
}

joinable!(exercise_entries -> exercise_types (exercise_type_id));
joinable!(exercise_entries -> meta_entries (meta_entry_id));
joinable!(meal_entries -> meal_types (meal_type_id));
joinable!(meal_entries -> meta_entries (meta_entry_id));
joinable!(meals -> foods (food_id));
joinable!(meals -> meal_entries (meal_entry_id));
joinable!(meta_entries -> users (user_id));
joinable!(sleep_entries -> meta_entries (meta_entry_id));

allow_tables_to_appear_in_same_query!(
    exercise_entries,
    exercise_types,
    foods,
    meal_entries,
    meal_types,
    meals,
    meta_entries,
    sleep_entries,
    users,
);
