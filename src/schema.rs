table! {
    links (id) {
        id -> Int8,
        url -> Varchar,
        redirect_count -> Int4,
        created_at -> Timestamp,
        expired_at -> Nullable<Timestamp>,
    }
}
