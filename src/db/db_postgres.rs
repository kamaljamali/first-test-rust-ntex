use crate::models::students_model::{Fields, StudentModel};
use crate::schemas::students_sch::students::dsl::students;
use crate::schemas::students_sch::students::{average_dip, family, fields, name, other};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::query_dsl::RunQueryDsl;
use diesel::result::Error;
use dotenv::dotenv;
use ntex::web;
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

pub async fn inserts_students(
    pool: &mut DbPool,
    item: web::types::Json<StudentModel>,
) -> QueryResult<bool> {
    let result = diesel::insert_into(students).values(&*item).execute(pool);
    Ok(true)
}

pub async fn update_students(
    pool: &mut DbPool,
    item: web::types::Json<StudentModel>,
) -> QueryResult<bool> {
    let updated_row = diesel::update(students.filter(name.eq(&item.name)))
        .set((
            name.eq(&item.name),
            family.eq(&item.family),
            average_dip.eq(&item.average_dip),
            fields.eq(&item.fields),
            other.eq(&item.other),
        ))
        .get_result::<StudentModel>(pool);
    match updated_row {
        Ok(res) => Ok(true),
        Err(err) => Err(Error::NotFound),
    }
}

pub async fn insert_field_students(
    pool: &mut DbPool,
    name_arg: String,
    items: web::types::Json<Fields>,
) -> QueryResult<bool> {
    let mut results: Vec<StudentModel> = students
        .limit(1)
        .select(StudentModel::as_select())
        .filter(name.eq(&name_arg))
        .load::<StudentModel>(pool)?;

    // چک کردن آیا دانشجو با نام داده شده وجود دارد یا نه
    if let Some(mut student) = results.pop() {
        // اضافه کردن رشته‌های جدید به لیست fields
        student.fields.extend(items.fields.clone());

        // به‌روزرسانی دانشجو در دیتابیس
        diesel::update(students.filter(name.eq(&name_arg)))
            .set(fields.eq(&student.fields))
            .execute(pool)?;
        Ok(true)
    } else {
        // اگر دانشجو با نام مشخص یافت نشد
        Err(diesel::result::Error::NotFound)
    }
}
