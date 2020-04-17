table! {
    exerciseentries (exerciseentryid) {
        exerciseentryid -> Int4,
        userid -> Nullable<Int4>,
        exercisetype -> Nullable<Int4>,
        starttime -> Timestamp,
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
        description -> Varchar,
    }
}

table! {
    mealentries (mealentryid) {
        mealentryid -> Int4,
        timestamp -> Timestamp,
        mealtype -> Nullable<Int4>,
        comments -> Nullable<Varchar>,
        userid -> Nullable<Int4>,
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
        userid -> Nullable<Int4>,
        starttime -> Timestamp,
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

joinable!(exerciseentries -> exercisetypes (exercisetype));
joinable!(exerciseentries -> users (userid));
joinable!(mealentries -> mealtypes (mealtype));
joinable!(mealentries -> users (userid));
joinable!(meals -> foods (foodid));
joinable!(meals -> mealentries (entryid));
joinable!(sleepentries -> users (userid));

allow_tables_to_appear_in_same_query!(
    exerciseentries,
    exercisetypes,
    foods,
    mealentries,
    meals,
    mealtypes,
    sleepentries,
    users,
);
