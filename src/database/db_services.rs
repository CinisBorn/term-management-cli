use crate::database::models::Term;
use rusqlite::{Connection, params};

pub fn add_term(db: &Connection, term: Term) -> Result<(), rusqlite::Error> {
    db.execute("
        INSERT OR IGNORE INTO terms (term, origin, type, definition) 
        VALUES (?1, ?2, ?3, ?4)
    ", params![term.term, term.origin, term.r#type, term.definition])?;
    
    Ok(())
}