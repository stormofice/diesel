// @generated automatically by Diesel CLI.

/// A module containing custom SQL type definitions
///
/// (Automatically generated by Diesel.)
pub mod sql_types {
    /// The `user_job` SQL type
    ///
    /// (Automatically generated by Diesel.)
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "user_job"))]
    pub struct UserJob;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::UserJob;

    /// Representation of the `users` table.
    ///
    /// (Automatically generated by Diesel.)
    users (id) {
        /// The `id` column of the `users` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Int4,
        /// The `job` column of the `users` table.
        ///
        /// Its SQL type is `UserJob`.
        ///
        /// (Automatically generated by Diesel.)
        job -> UserJob,
    }
}
