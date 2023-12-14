use diesel::deserialize::{self, FromSql};
use diesel::pg::Pg;
use diesel::serialize::{self, Output, ToSql};
use diesel::sql_types::Jsonb;
use serde::{Serialize, Deserialize};
use serde_json::Value;
use std::io::Write;

#[derive(Debug, Clone, PartialEq, FromSqlRow, AsExpression, Serialize, Deserialize)]
pub struct MyJsonValue(pub serde_json::Value);

#[derive(Debug, Serialize, Deserialize)]
pub struct MyJsonb(pub diesel::sql_types::Jsonb);

impl Serialize for MyJsonb {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        fn serialize_myjsonb<S>(value: &MyJsonb, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            value.0.serialize(serializer)
        }

        serialize_myjsonb(self, serializer)
    }
}

#[derive(Debug, Clone, PartialEq, FromSqlRow, AsExpression, Serialize, Deserialize)]
#[sql_type = "Jsonb"]
pub struct JsonbValue(pub Value);

impl ToSql<Jsonb, Pg> for MyJsonb {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        let json_str = serde_json::to_string(&self.0)?;
        ToSql::<diesel::sql_types::Text, Pg>::to_sql(&json_str, out)
    }
}

impl FromSql<Jsonb, Pg> for MyJsonb {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        let json_str = String::from_utf8_lossy(bytes.unwrap()).to_string();
        let value = serde_json::from_str(&json_str)?;
        Ok(Self(JsonbValue(value)))
    }
}
