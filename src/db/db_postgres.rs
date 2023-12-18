use crate::models::students_model::StudentModel;
use crate::schemas::students_sch::students::dsl::students;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::query_dsl::RunQueryDsl;
use dotenv::dotenv;
use std::env;

pub type DbPool = PgConnection;

pub fn establish_connection() -> DbPool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub async fn perform_async_query(pool: &mut DbPool) {
    let result = diesel::sql_query("SELECT 1 + 1")
        .execute(pool)
        .ok()
        .expect("Error executing query asynchronously.");

    println!("perform_async_query. Rows affected: {}", result);
}

pub async fn get_all_students2(pool: &mut DbPool) -> QueryResult<Vec<StudentModel>> {
    let results = students
    .limit(5)
    .select(StudentModel::as_select())
    .load::<StudentModel>(pool)
    .expect("Error loading posts");
Ok(results)
}
