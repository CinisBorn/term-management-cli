use dotenvy;
use rusqlite::Connection;

pub mod db_services;
pub mod models;

pub fn start_db() -> Result<Connection, String> {
    use std::env;

    dotenvy::dotenv().map_err(|e| e.to_string())?;

    let path: String = env::var("DB_PATH")
        .or_else(|_| Ok::<String, String>(String::from("./temp_db.sqlite")))
        .unwrap(); // unwrap is safe  here why this operation always return a "Ok" 

    let db: Connection = Connection::open(&path).map_err(|e| e.to_string())?;

    check_path(&path);
    generate_tables(&db).map_err(|e| e.to_string())?;

    Ok(db)
}

fn generate_tables(db: &Connection) -> Result<(), rusqlite::Error> {
    db.execute(
        "
        CREATE TABLE IF NOT EXISTS terms (
            id INTEGER PRIMARY KEY,
            term TEXT UNIQUE NOT NULL,
            origin TEXT DEFAULT 'empty', 
            type TEXT DEFAULT 'empty',
            definition TEXT NOT NULL
        )
    ",
        (),
    )?;

    db.execute(
        "
        CREATE TABLE IF NOT EXISTS terms_relation (
            id INTEGER PRIMARY KEY, 
            from_id INTEGER NOT NULL,
            to_id INTEGER NOT NULL,
            relation TEXT NOT NULL,
            
            FOREIGN KEY(from_id) REFERENCES terms(id),
            FOREIGN KEY(to_id) REFERENCES terms(id)
        )
    ",
        (),
    )?;

    Ok(())
}

fn check_path(path: &String) {
    if path == "/temp_db.sqlite" {
        println!("⚠ Database not found! Using a temporary database for store the new data");
    } else {
        println!("✅ Database loaded successfully");
    }
}
