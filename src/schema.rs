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
        email -> Text,
        pw -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    histories,
    users,
);
