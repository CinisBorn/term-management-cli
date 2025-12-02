use helpers::normalize_args;

mod helpers;

pub fn manage_operation(args: Vec<String>) -> Result<(), String> {
    let formatted_args = normalize_args(args)?;
    
    match formatted_args[0].as_str() {
        "add" => todo!(),
        "remove" => todo!(),
        "update" => todo!(),
        "get" => todo!(),
        "check" => todo!(),
        "relations_for" => todo!(),
        _ => println!("ops")
    }

    Ok(())
}
