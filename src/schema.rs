table! {
    messages (id) {
        id -> Int4,
        text -> Text,
        from_user_id -> Int4,
        to_user_id -> Int4,
        created_at -> Text,
    }
}

table! {
    users (id) {
        id -> Int4,
        name -> Text,
        password_hash -> Text,
        created_at -> Text,
        role -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    messages,
    users,
);
