pub mod users;

#[database("diesel_postgres_pool")]
pub struct Database(diesel::PgConnection);
