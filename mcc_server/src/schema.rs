table! {
    recipes (id) {
        id -> Integer,
        name -> Text,
        json_data -> Text,
        image_file -> Nullable<Text>,
        lang -> Nullable<Text>,
        original_id -> Nullable<Integer>,
        recipe_type -> Nullable<Text>,
        is_custom -> Nullable<Bool>,
    }
}
