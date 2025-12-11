use crate::cli::cli_models::TermRelationUserInput;
use crate::database::models::{Term, TermRelation, TermRelationView};
use rusqlite::{Connection, params};

pub fn add_term(db: &Connection, term: Term) -> Result<(), rusqlite::Error> {
    db.execute(
        "
        INSERT OR IGNORE INTO terms (term, more_information) 
        VALUES (?1, ?2)
    ",
        params![term.term, term.more_information],
    )?;

    Ok(())
}

pub fn get_term(db: &Connection, term: String) -> Result<Term, rusqlite::Error> {
    let mut query = db.prepare("SELECT term, more_information FROM terms WHERE term = ?1")?;
    let content = query.query_row([term], |r| {
        Ok(Term {
            term: r.get(0)?,
            more_information: r.get(1)?,
        })
    })?;

    Ok(content)
}

pub fn get_term_id(db: &Connection, term: String) -> Result<i64, rusqlite::Error> {
    let mut query = db.prepare("SELECT id FROM terms WHERE term = ?1")?;
    let id: i64 = query.query_row(params![term], |r| Ok(r.get(0)?))?;

    Ok(id)
}

pub fn remove_term(db: &Connection, id: i64) -> Result<(), rusqlite::Error> {
    db.execute("DELETE FROM terms WHERE id = ?1", params![id])?;

    Ok(())
}

pub fn update_term(db: &Connection, term: Term, id: i64) -> Result<(), rusqlite::Error> {
    db.execute(
        "
        UPDATE terms 
        SET
            term = COALESCE(NULLIF(?1, ''), term),
            more_information = COALESCE(NULLIF(?2, ''), more_information)
        WHERE 
        id = ?3
    ",
        params![term.term, term.more_information, id],
    )?;

    Ok(())
}

pub fn check_term(db: &Connection, term: String) -> Result<bool, rusqlite::Error> {
    let mut query = db.prepare("SELECT term FROM terms WHERE term = ?1")?;
    let result: Result<String, _> = query.query_row([term], |r| Ok(r.get(0)?));

    match result {
        Ok(_) => Ok(true),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(false),
        Err(e) => Err(e),
    }
}

pub fn create_relation(
    db: &Connection,
    input: TermRelationUserInput,
) -> Result<(), rusqlite::Error> {
    let from_term_id = get_term_id(db, input.from_term)?;
    let to_term_id = get_term_id(db, input.to_term)?;

    db.execute(
        "
        INSERT OR IGNORE INTO terms_relation (from_id, to_id)
        VALUES (?1, ?2)
    ",
        params![from_term_id, to_term_id],
    )?;

    Ok(())
}

pub fn get_term_by_id(db: &Connection, id: i64) -> Result<String, rusqlite::Error> {
    let mut query = db.prepare("SELECT term FROM terms WHERE id = ?1")?;
    let term: String = query.query_row([id], |r| Ok(r.get(0)?))?;

    Ok(term)
}

pub fn get_relation(
    db: &Connection,
    term: String,
) -> Result<Vec<TermRelationView>, rusqlite::Error> {
    let term_id = get_term_id(db, term)?;
    let mut query = db.prepare(
        "
      SELECT from_id, to_id FROM terms_relation WHERE from_id = ?1 OR to_id = ?1  
    ",
    )?;
    let rows = query.query_map([term_id], |r| {
        Ok(TermRelation {
            from_id: r.get(0)?,
            to_id: r.get(1)?,
        })
    })?;

    let mut relations_list: Vec<TermRelation> = Vec::new();
    let mut relations_list_view: Vec<TermRelationView> = Vec::new();

    for r in rows {
        relations_list.push(r.unwrap());
    }

    for r in relations_list {
        let from = get_term_by_id(db, r.from_id)?;
        let to = get_term_by_id(db, r.to_id)?;

        relations_list_view.push(TermRelationView {
            from_id: from,
            to_id: to,
        });
    }

    Ok(relations_list_view)
}
