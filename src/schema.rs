table! {
    entries (entryid) {
        entryid -> Int4,
        userid -> Nullable<Int4>,
        mealentryid -> Nullable<Int4>,
        exerciseentryid -> Nullable<Int4>,
        sleepentryid -> Nullable<Int4>,
        timestamp -> Timestamp,
    }
}

table! {
    exerciseentries (exerciseentryid) {
        exerciseentryid -> Int4,
        exercisetype -> Int4,
        endtime -> Timestamp,
        comments -> Nullable<Varchar>,
    }
}

table! {
    exercisetypes (exercisetypeid) {
        exercisetypeid -> Int4,
        name -> Varchar,
    }
}

table! {
    foods (foodid) {
        foodid -> Int4,
        name -> Varchar,
    }
}

table! {
    mealentries (mealentryid) {
        mealentryid -> Int4,
        mealtype -> Int4,
        comments -> Nullable<Varchar>,
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
    sleepentries (sleepentryid) {
        sleepentryid -> Int4,
        endtime -> Timestamp,
        comments -> Nullable<Varchar>,
    }
}

table! {
    users (userid) {
        userid -> Int4,
        username -> Varchar,
        password -> Varchar,
    }
}

joinable!(entries -> exerciseentries (exerciseentryid));
joinable!(entries -> mealentries (mealentryid));
joinable!(entries -> sleepentries (sleepentryid));
joinable!(entries -> users (userid));
joinable!(exerciseentries -> exercisetypes (exercisetype));
joinable!(mealentries -> mealtypes (mealtype));
joinable!(meals -> foods (foodid));
joinable!(meals -> mealentries (entryid));

allow_tables_to_appear_in_same_query!(
    entries,
    exerciseentries,
    exercisetypes,
    foods,
    mealentries,
    meals,
    mealtypes,
    sleepentries,
    users,
);
