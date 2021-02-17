table! {
    posts (id) {
        id -> Integer,
        title -> Text,
        body -> Text,
        published -> Bool,
        ctime -> Timestamp,
        owner_id -> Integer,
    }
}

table! {
    users (id) {
        id -> Integer,
        first_name -> Text,
        last_name -> Text,
        ctime -> Timestamp,
    }
}

joinable!(posts -> users (owner_id));

allow_tables_to_appear_in_same_query!(
    posts,
    users,
);
