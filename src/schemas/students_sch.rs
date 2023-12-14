table! {
    students(average_dip) {
        name -> Varchar,
        family -> Varchar,
        average_dip -> Integer,
        // other -> serde_json::Value,
        fields -> Array<Text>,
        // _id -> Timestamptz,
        }
}
