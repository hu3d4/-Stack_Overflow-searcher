table! {
    histories (id) {
        id -> Int4,
        username -> Text,
        input -> Text,
        done -> Bool,
    }
}

table! {
    users (userid) {
        userid -> Int4,
        username -> Text,
        login_status -> Bool,
    }
}

allow_tables_to_appear_in_same_query!(histories, users,);
