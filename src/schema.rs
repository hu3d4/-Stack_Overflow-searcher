table! {
    histories (id) {
        id -> Int4,
        input -> Text,
        done -> Bool,
    }
}

table! {
    users (id) {
        id -> Int4,
        users_name -> Text,
        login_status -> Bool,
    }
}

allow_tables_to_appear_in_same_query!(
    histories,
    users,
);
