table! {
    entries (id) {
        id -> Int4,
        user_id -> Int4,
        timestamp -> Timestamp,
        comments -> Nullable<Varchar>,
    }
}

table! {
    exercise_entries (id) {
        id -> Int4,
        entry_id -> Int4,
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
        entry_id -> Int4,
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
    sleep_entries (id) {
        id -> Int4,
        entry_id -> Int4,
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

joinable!(entries -> users (user_id));
joinable!(exercise_entries -> entries (entry_id));
joinable!(exercise_entries -> exercise_types (exercise_type_id));
joinable!(meal_entries -> entries (entry_id));
joinable!(meal_entries -> meal_types (meal_type_id));
joinable!(meals -> foods (food_id));
joinable!(meals -> meal_entries (meal_entry_id));
joinable!(sleep_entries -> entries (entry_id));

allow_tables_to_appear_in_same_query!(
    entries,
    exercise_entries,
    exercise_types,
    foods,
    meal_entries,
    meal_types,
    meals,
    sleep_entries,
    users,
);
