use diesel::RunQueryDsl;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenv::dotenv;
use std::env;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

pub fn create_pool() -> DbPool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let cloned_database_url = database_url.clone();
    println!("Database URL: {}", cloned_database_url);

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    // Use Pool::builder().build(manager) to create and return the pool
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}

pub async fn perform_async_query(pool: &DbPool) {
    let connection = pool.get().expect("Failed to get connection from pool.");

    let result = diesel::sql_query("SELECT 1 + 1").execute(&connection)
    .ok()
    .expect("Error executing query asynchronously.");

    println!("Query executed. Rows affected: {}", result);
}
