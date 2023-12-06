use chrono::{DateTime,Local};
use serde::{Deserialize, Serialize};
use std::net::{SocketAddr};

#[derive(Deserialize,Serialize)]
pub struct MyObj {
    pub first_name: String,
    pub last_name: String,
    pub id: i32,
}

#[derive(Deserialize,Serialize)]
pub struct PingObject{
    pub message:String,
    pub date_time:DateTime<Local>,
    pub ip:Option<SocketAddr>,
    pub url:String,
}
