table! {
    users (id) {
        id -> Int4,
        first_name -> Nullable<Varchar>,
        last_name -> Nullable<Varchar>,
        username -> Varchar,
        email -> Text,
        passwd -> Text,
        created_at -> Timestamp,
    }
}
