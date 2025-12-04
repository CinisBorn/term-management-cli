use cli::manage_operation;
use database::start_db;
use std::env;

mod cli;
pub mod database;

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    let db = start_db()?;

    match manage_operation(args, &db) {
        Ok(m) => {
            println!("{}", m);
            Ok(())
        }
        Err(e) => {
            eprintln!("{}", e);
            Err(e)
        }
    }
}
