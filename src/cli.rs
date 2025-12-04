use crate::{cli::cli_services::get_relation_input, database::db_services::{add_term, check_term, create_relation, get_term, get_term_id, remove_term, update_term}};
use helpers::normalize_args;
use cli_services::get_input;
use rusqlite::Connection;

mod cli_services;
mod helpers;
pub mod cli_models;

pub fn manage_operation(args: Vec<String>, db: &Connection) -> Result<String, String> {
    let formatted_args = normalize_args(args)?;
    
    match formatted_args[0].as_str() {
        "add" => {
            let user_data = get_input().map_err(|e| e.kind().to_string())?;
            let result = add_term(db, user_data);
            
            match result {
                Ok(_) => Ok("✅ The term was added successfully".to_string()),
                Err(_) => Err("❌ The term was not added successfully".to_string())
            }
        },
        "remove" => {
            let id = get_term_id(db, formatted_args[1].clone()).map_err(|e| e.to_string())?;
            let result = remove_term(db, id);
            
            match result {
                Ok(_) => Ok("✅ Term deleted successfully".to_string()),
                Err(_) => Err("❌ The terms was not deleted".to_string())
            }
        },
        "update" => {
            let id = get_term_id(db, formatted_args[1].clone()).map_err(|e| e.to_string())?;
            let user_input = get_input().map_err(|e| e.to_string())?;
            let result = update_term(db, user_input, id);
            
            match result {
                Ok(_) => Ok("✅ Term updated successfully".to_string()),
                Err(_) => Err("❌ Term wasn't added successfully.".to_string())
            }
        },
        "get" => {
            let term = get_term(db, formatted_args[1].clone()).map_err(|e| e.to_string());
            
            match term {
                Ok(t) => {
                    println!("Term: {}; Type: {}; Origin: {}; ", t.term, t.r#type, t.origin);
                    println!("Definition: {}", t.definition);
                    
                    Ok("✅ Term was got successfully".to_string())
                },
                Err(_) => Err("❌ The term was not got successfully".to_string())
            }
        },
        "check" => {
            let result = check_term(db, formatted_args[1].clone()).map_err(|e| e.to_string());
            
            match result {
                Ok(r) => {
                    if r { Ok("✅ The term already exists!".to_string())} 
                    else { Ok("❗ The term was not found!".to_string())}
                },
                Err(_) => Err("❌ Internal Error".to_string())
            }
        },
        "relations_for" => todo!(),
        "relation" => {
            let input = get_relation_input().map_err(|e| e.to_string())?;
            let result = create_relation(db, input);
            
            match result {
                Ok(_) => Ok("✅ The relation as estabilished successfully".to_string()),
                Err(_) => Err("❌  The relation was not estabilished successfully".to_string())
             }
        }
        _ => todo!()
    }
}
