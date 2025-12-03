use crate::database::models::Term;
use std::io;

pub fn get_input() -> Result<Term, io::Error> {
    let mut term = String::new();
    let mut origin = String::new();
    let mut r#type = String::new();
    let mut definition = String::new();
    
    println!("👉 Type the term name"); io::stdin().read_line(&mut term)?; 
    println!("👉 Type the type of term"); io::stdin().read_line(&mut r#type)?; 
    println!("👉 Type the origin of term"); io::stdin().read_line(&mut origin)?; 
    println!("👉 Type the definition of term"); io::stdin().read_line(&mut definition)?; 
    
    Ok(Term { 
        definition: definition.trim().to_string().to_lowercase(), 
        term: term.trim().to_string().to_lowercase(),
        origin: origin.trim().to_string().to_lowercase(),
        r#type: r#type.trim().to_string().to_lowercase()
    })
}