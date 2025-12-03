use crate::database::models::Term;
use rusqlite::{Connection, params};

// there is a bug where terms with white spaces will not work. The function "normalize_args"
// will remove white space. Fix it later. 
pub fn add_term(db: &Connection, term: Term) -> Result<(), rusqlite::Error> {
    db.execute("
        INSERT OR IGNORE INTO terms (term, origin, type, definition) 
        VALUES (?1, ?2, ?3, ?4)
    ", params![term.term, term.origin, term.r#type, term.definition])?;
    
    Ok(())
}

pub fn get_term(db: &Connection, term: String) -> Result<Term, rusqlite::Error> {
    let mut query = db.prepare("SELECT term, origin, type, definition FROM terms WHERE term = ?1")?;
    let content = query.query_row([term], |r| {
        Ok(Term {
            term: r.get(0)?,
            origin: r.get(1)?,
            r#type: r.get(2)?,
            definition: r.get(3)?
        })
    })?;
    
    Ok(content)
}