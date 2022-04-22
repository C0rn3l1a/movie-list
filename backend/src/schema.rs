table! {
    movies (id) {
        id -> Int4,
        owner_id -> Int4,
        name -> Varchar,
        seen -> Bool,
    }
}

table! {
    owners (id) {
        id -> Int4,
        name -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    movies,
    owners,
);
