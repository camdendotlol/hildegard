use crate::{db::connection::db_connect, models::Script};
use diesel::*;

#[tauri::command]
pub async fn count() -> String {
    use crate::schema::scripts::dsl::*;

    let mut script_vec = Vec::new();

    let mut connection = db_connect();

    match scripts.load::<Script>(&mut connection) {
        Ok(mut list) => script_vec.append(&mut list),
        _ => (),
    }

    format!("{}", script_vec.len())
}