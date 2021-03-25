table! {
    links (id) {
        id -> Int8,
        url -> Varchar,
        created_at -> Timestamp,
        expired_at -> Nullable<Timestamp>,
    }
}
