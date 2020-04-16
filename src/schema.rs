table! {
    entries (entryid) {
        entryid -> Int4,
        timestamp -> Timestamp,
        mealtype -> Nullable<Int4>,
        comments -> Nullable<Varchar>,
        userid -> Nullable<Int4>,
    }
}

table! {
    foods (foodid) {
        foodid -> Int4,
        name -> Varchar,
        description -> Varchar,
    }
}

table! {
    meals (foodid, entryid) {
        foodid -> Int4,
        entryid -> Int4,
        qty -> Int4,
    }
}

table! {
    mealtypes (mealtypeid) {
        mealtypeid -> Int4,
        name -> Varchar,
    }
}

table! {
    users (userid) {
        userid -> Int4,
        username -> Varchar,
        password -> Varchar,
    }
}

joinable!(entries -> mealtypes (mealtype));
joinable!(entries -> users (userid));
joinable!(meals -> entries (entryid));
joinable!(meals -> foods (foodid));

allow_tables_to_appear_in_same_query!(
    entries,
    foods,
    meals,
    mealtypes,
    users,
);
