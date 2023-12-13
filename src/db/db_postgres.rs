use crate::models::students_model::StudentModel;
use crate::schemas::students_sch::students::table;
use diesel::pg::PgConnection;
use diesel::query_dsl::RunQueryDsl;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::QueryResult;
use dotenv::dotenv;
use lazy_static::lazy_static;
use r2d2::PooledConnection;
use std::env;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

lazy_static! {
    static ref POOL: DbPool = {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.")
    };
}

pub fn get_db_pool() -> &'static DbPool {
    &POOL
}

pub fn create_pool() -> DbPool {
    POOL.clone()
}
pub async fn perform_async_query(pool: &DbPool) {
    let connection = pool.get().expect("Failed to get connection from pool.");

    let result = diesel::sql_query("SELECT 1 + 1")
        .execute(&connection)
        .ok()
        .expect("Error executing query asynchronously.");

    println!("perform_async_query. Rows affected: {}", result);
}

pub async fn get_all_students(
    connection: &PooledConnection<ConnectionManager<PgConnection>>,
) -> QueryResult<Vec<StudentModel>> {
    let students = table.load::<StudentModel>(connection)?;
    Ok(students)
}

pub async fn get_all_students2(connection: &DbPool) -> QueryResult<Vec<StudentModel>> {
    let pooled_connection = connection.get().expect("ERRRO");
    let students = table.load::<StudentModel>(&pooled_connection)?;
    Ok(students)
}


// pub async fn get_all_students(pool: &DbPool) {
//     let connection = pool.get().expect("Failed to get connection from pool.");

//     let result: Vec<Student> = diesel::sql_query("SELECT * from students")
//         .load::<Student>(&connection)
//         .expect("Error");
//     // .ok()
//     // .expect("Error executing query asynchronously.");
// }
