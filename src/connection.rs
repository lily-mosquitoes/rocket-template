use rocket_sync_db_pools::database;

#[database("postgres")]
pub struct PostgresPool(diesel::PgConnection);
