use crate::schemas::students_sch::students::{self};
use diesel::prelude::*;
use first_test::AsJsonb;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, ToSchema, AsJsonb)]
pub struct Fields {
    pub fields: Vec<String>,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, ToSchema, AsJsonb)]
pub struct OtherStudent {
    pub test: String,
    pub admin: bool,
    pub birthdate: String,
}

#[derive(
    Clone, Queryable, Serialize, Deserialize, PartialEq, Debug, ToSchema, Selectable, Insertable,
)]
#[diesel(table_name = students)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct StudentModel {
    pub name: String,
    pub family: String,
    pub average_dip: i32,
    pub other: OtherStudent,
    pub fields: Vec<String>,
    // pub _id: String,
}
