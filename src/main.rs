use database::start_db;

mod database;

fn main() -> Result<(), String>{
    let db = start_db()?;
    
    Ok(())
}
