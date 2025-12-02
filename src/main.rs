use std::env;
use database::start_db;
use cli::manage_operation;

pub mod database;
mod cli;

fn main() -> Result<(), String>{
    let args: Vec<String> = env::args().collect();
    let db = start_db()?;
    
    manage_operation(args, &db)?;
    
    Ok(())
}
