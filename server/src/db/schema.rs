table! {
    datasets (id) {
        id -> Int4,
        plot_id -> Int4,
        x -> Nullable<Float8>,
        y -> Nullable<Float8>,
    }
}

table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}

allow_tables_to_appear_in_same_query!(datasets, posts,);
