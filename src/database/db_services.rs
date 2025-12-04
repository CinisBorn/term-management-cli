use crate::database::models::{Term, TermRelation};
use crate::cli::cli_models::TermRelationUserInput;
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
    let mut query = db.prepare(
        "SELECT term, origin, type, definition FROM terms WHERE term = ?1"
    )?;
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

pub fn get_term_id(db: &Connection, term: String) -> Result<i64, rusqlite::Error> {
    let mut query = db.prepare("SELECT id FROM terms WHERE term = ?1")?;
    let id: i64 = query.query_row(params![term], |r| {
        Ok(r.get(0)?)
    })?;
    
    Ok(id)
}

pub fn remove_term(db: &Connection, id: i64) -> Result<(), rusqlite::Error> {
    db.execute("DELETE FROM terms WHERE id = ?1", params![id])?;
    
    Ok(())
}

pub fn update_term(db: &Connection, term: Term, id: i64) -> Result<(), rusqlite::Error> {
    db.execute("
        UPDATE terms 
        SET
            term = COALESCE(NULLIF(?1, ''), term),
            definition = COALESCE(NULLIF(?2, ''), definition),
            type = COALESCE(NULLIF(?3, ''), type),
            origin = COALESCE(NULLIF(?4, ''), origin)
        WHERE 
        id = ?5
    ", params![term.term, term.definition, term.r#type, term.origin, id])?;

    Ok(())
}

pub fn check_term(db: &Connection, term: String) -> Result<bool, rusqlite::Error> {
    let mut query = db.prepare("SELECT term FROM terms WHERE term = ?1")?;
    let result: Result<String, _> = query.query_row([term], |r| {
        Ok(r.get(0)?)
    });
    
    match result {
        Ok(_) => Ok(true),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(false), 
        Err(e) => Err(e)
    }
}

pub fn create_relation(db: &Connection, input: TermRelationUserInput) -> Result<(), rusqlite::Error> {
    let from_term_id = get_term_id(db, input.from_term)?;
    let to_term_id = get_term_id(db, input.to_term)?;
    
    db.execute("
        INSERT OR IGNORE INTO terms_relation (from_id, to_id, relation)
        VALUES (?1, ?2, ?3)
    ", params![from_term_id, to_term_id, input.relation])?;
    
    Ok(())
}

pub fn get_term_by_id(db: &Connection, id: i64) -> Result<String, rusqlite::Error> {
    let mut query = db.prepare("SELECT term FROM terms WHERE id = ?1")?;
    let term: String = query.query_row([id], |r| {
        Ok(r.get(0)?)
    })?;
    
    Ok(term)
}

pub fn get_relation(db: &Connection, term: String) -> Result<(), rusqlite::Error> {
    let term_id = get_term_id(db, term)?;
    let mut query = db.prepare("
      SELECT from_id, to_id FROM terms_relation WHERE from_id = ?1 OR to_id = ?1  
    ")?;
    let rows = query.query_map([term_id], |r| {
        Ok(TermRelation {
            from_id: r.get(0)?,
            to_id: r.get(1)?,
            relation: String::new(),
        })
    })?; 
    
    for r in rows {
        let relation = r.unwrap().clone();
        
        let from = get_term_by_id(db, relation.from_id)?;
        let to = get_term_by_id(db, relation.to_id)?;
        
        println!("{} => {}", from,to);
    }
    
    Ok(())
}