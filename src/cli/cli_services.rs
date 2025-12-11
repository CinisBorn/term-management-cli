use tabled::settings::{Alignment, Style, object::Columns};
use crate::cli::cli_models::TermRelationUserInput;
use tabled::Table;
use crate::database::models:: Term;
use std::io;

pub fn get_input() -> Result<Term, io::Error> {
    let mut term = String::new();
    let mut info = String::new();

    println!("👉 Type the term name");
    io::stdin().read_line(&mut term)?;
    println!("👉 Type where you can find more information about this term");
    io::stdin().read_line(&mut info)?;


    Ok(Term {
        term: term.trim().to_string().to_lowercase(),
        more_information: info.trim().to_string().to_lowercase(),
    })
}

pub fn get_relation_input() -> Result<TermRelationUserInput, io::Error> {
    let mut term_from = String::new();
    let mut term_to = String::new();

    println!("👉 Create a relation from term: ");
    io::stdin().read_line(&mut term_from)?;
    println!("👉 To the term: ");
    io::stdin().read_line(&mut term_to)?;

    Ok(TermRelationUserInput {
        from_term: term_from.trim().to_string().to_lowercase(),
        to_term: term_to.trim().to_string().to_lowercase(),
    })
}

pub fn view_data(source: &Term) {
    let mut table = Table::new(vec![source]);
    
    table.modify(Columns::first(), Alignment::right());
    table.with(Style::modern_rounded());
    
    println!("{}", table);
}