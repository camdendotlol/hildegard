use diesel::prelude::*;
use serde::Serialize;

#[derive(Serialize, Queryable)]
pub struct Script {
    pub id: i32,
    pub name: String,
    pub description: Option<String>
}

#[derive(Serialize, Queryable)]
pub struct Symbol {
    pub id: i32,
    pub unicode_number: i32,
    pub is_composite: bool
}
