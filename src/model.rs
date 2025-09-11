use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::schema::member;

#[derive(Queryable, Serialize)]
pub struct Member {
    pub id:i32,
    pub f_name: String,
    pub m_name: String,
    pub l_name: String,
    pub email: String,
    pub address: String,
    pub age: i32,
}

#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = member)]
pub struct NewMember {
    pub f_name: String,
    pub m_name: String,
    pub l_name: String,
    pub email: String,
    pub address: String,
    pub age: i32,
}

#[derive(Deserialize, AsChangeset)]
#[diesel(table_name = member)]
pub struct UpdateMember {
    pub f_name: Option<String>,
    pub m_name: Option<String>,
    pub l_name: Option<String>,
    pub email: Option<String>,
    pub address: Option<String>,
    pub age: Option<i32>,
}