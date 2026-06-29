// @generated automatically by Diesel CLI.

diesel::table! {
    employees (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        age -> Nullable<Int4>,
        #[max_length = 255]
        role -> Nullable<Varchar>,
        #[max_length = 50]
        status -> Varchar,
    }
}
