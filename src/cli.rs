use crate::database::{db_services::{
    add_term, check_term, create_relation, get_relation, get_term, get_term_id, remove_term,
    update_term,
}, models::Term};
use cli_services::{get_input, get_relation_input, view_data};
use helpers::normalize_args;
use rusqlite::Connection;

pub mod cli_models;
mod cli_services;
mod helpers;

pub fn manage_operation(args: Vec<String>, db: &Connection) -> Result<String, String> {
    let formatted_args = normalize_args(args)?;

    match formatted_args[0].as_str() {
        "add" => {
            let user_data = get_input().map_err(|e| format!("Input Error: {}", e))?;
            add_term(db, user_data).map_err(|e| format!("Database Error: {}", e))?;

            Ok(String::from("✅ The term was added successfully")) 
        }
        "remove" => {
            let term = formatted_args[1].clone();
            let id = get_term_id(db, term).map_err(|e| format!("❌ Database Error: {}", e))?;
            remove_term(db, id).map_err(|e| format!("❌ Database Error: {}", e))?;

            Ok(String::from("✅ Term deleted successfully"))
        }
        "update" => {
            let term = formatted_args[1].clone();
            let id = get_term_id(db, term).map_err(|e| format!("❌ Database Error: {}", e))?;
            let input = get_input().map_err(|e| format!("❌ Input Error: {}", e))?;
            update_term(db, input, id).map_err(|e| format!("❌ Database Error: {}", e))?;

            Ok(String::from("✅ Term updated successfully"))
        }
        "get" => {
            let term =  formatted_args[1].clone();
            let content = get_term(db, term).map_err(|e| format!("❌ Database Error: {}", e))?;

            view_data(&Term { 
                term: content.term, 
                more_information: content.more_information
            });
            
            Ok(String::from("✅ Term was got successfully"))
        }
        "check" => {
            let term = formatted_args[1].clone();
            let result = check_term(db, term).map_err(|e| format!("Database Error: {}", e))?;

            if result { Ok(String::from("✅ The term already exists"))} 
            else { Ok(String::from("❗ The term was not found"))}
        }
        "relations" => {
            let term = formatted_args[1].clone();
            get_relation(db, term).map_err(|e| format!("Database Error: {}", e))?;
            
            Ok(String::from ("✅ All results was got successfully"))
        }
        "relation" => {
            let input = get_relation_input().map_err(|e| format!("Input Error: {}", e))?;
            create_relation(db, input).map_err(|e| format!("❌Database Error: {}", e))?;

            Ok(String::from("✅ The relation was created successfully"))
        }
        _ => todo!(),
    }
}
