use diesel::prelude::*;

pub fn db_connect() -> SqliteConnection {
    let database_url = String::from("hildegard.db");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
