use crate::database::db_services::{add_term, get_term, get_term_id, remove_term};
use helpers::normalize_args;
use cli_services::get_input;
use rusqlite::Connection;

mod cli_services;
mod helpers;

pub fn manage_operation(args: Vec<String>, db: &Connection) -> Result<(), String> {
    let formatted_args = normalize_args(args)?;
    
    match formatted_args[0].as_str() {
        "add" => {
            let user_data = get_input().map_err(|e| e.kind().to_string())?;
            let result = add_term(db, user_data);
            
            match result {
                Ok(_) => println!("✅ The term was added successfully"),
                Err(e) => println!("❌ The term was not added successfully. {}", e)
            }
        },
        "remove" => {
            let id = get_term_id(db, formatted_args[1].clone()).map_err(|e| e.to_string())?;
            let result = remove_term(db, id);
            
            match result {
                Ok(_) => println!("✅ Term deleted successfully"),
                Err(_) => eprintln!("❌ The terms was not deleted")
            }
        },
        "update" => todo!(),
        "get" => {
            let term = get_term(db, formatted_args[1].clone()).map_err(|e| e.to_string())?;
            
            println!("Term: {}; Type: {}; Origin: {}; ", term.term, term.r#type, term.origin);
            println!("Definition: {}", term.definition)
        },
        "check" => todo!(),
        "relations_for" => todo!(),
        _ => println!("ops")
    }

    Ok(())
}
