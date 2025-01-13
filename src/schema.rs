diesel::table! {
    file (id) {
        #[max_length = 36]
        id -> Varchar,
        #[max_length = 128]
        name -> Varchar,
        datatype -> Int4,
    }
}

diesel::table! {
    file_tags (id) {
        id -> Int4,
        #[max_length = 36]
        file_id -> Varchar,
        tag_value -> Nullable<Int4>,
    }
}

diesel::table! {
    tag (id) {
        id -> Int4,
        #[max_length = 128]
        name -> Varchar,
    }
}

diesel::table! {
    tag_values (id) {
        id -> Int4,
        tag_id -> Nullable<Int4>,
        #[max_length = 128]
        value -> Varchar,
    }
}

diesel::joinable!(file_tags -> file (file_id));
diesel::joinable!(tag_values -> tag (tag_id));

diesel::allow_tables_to_appear_in_same_query!(
    file,
    file_tags,
    tag,
    tag_values,
);
