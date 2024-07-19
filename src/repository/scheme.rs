// @generated automatically by Diesel CLI.

diesel::table! {
    categories (id) {
        id -> Int4,
        uuid -> Text,
        name -> Text,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    recipes (id) {
        id -> Int4,
        uuid -> Text,
        cover -> Bytea,
        title -> Text,
        description -> Text,
        time_to_cook -> Int8,
        difficulty -> Text,
        servings -> Int8,
        category_id -> Text,
        ingredients -> Array<Nullable<Text>>,
        guideline -> Text,
        updated_at -> Timestamp,
        #[sql_name = "nutrients.proteins"]
        nutrients_proteins -> Int8,
        #[sql_name = "nutrients.fats"]
        nutrients_fats -> Float8,
        #[sql_name = "nutrients.carbohydrates"]
        nutrients_carbohydrates -> Float8,
        #[sql_name = "nutrients.fiber"]
        nutrients_fiber -> Int8,
        #[sql_name = "nutrients.kcal"]
        nutrients_kcal -> Int8,
    }
}

diesel::allow_tables_to_appear_in_same_query!(categories, recipes,);
