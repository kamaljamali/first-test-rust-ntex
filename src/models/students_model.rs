use diesel::Queryable;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
#[derive(Clone, Queryable, PartialEq, Debug, Serialize, Deserialize, ToSchema)]
pub struct OtherStudent {
    pub test: String,
    pub admin: String,
    pub birthdate: String,
}

#[derive(Clone, Queryable, PartialEq, Debug, Serialize, Deserialize, ToSchema)]
pub struct StudentModel {
    pub name: String,
    pub family: String,
    pub average_dip:i32
    // pub other: serde_json::Value,
    // pub fields: serde_json::Value,
    // pub _id: String,
}
