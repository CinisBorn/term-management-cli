pub fn normalize_args(mut args: Vec<String>) -> Result<Vec<String>, String> {
    if args.is_empty() {
        return Err("No arguments found".to_string());
    }

    let mut clear_args: Vec<String> = Vec::new();
    let mut normalized_vec: Vec<String> = Vec::new();

    println!("{}", args.remove(0)); // remove the default arg from OS 

    for a in args {
        clear_args.push(a.trim().to_lowercase());
    }

    for a in clear_args {
        let chars: Vec<char> = a.chars().collect();
        let chars_in_string: Vec<String> = chars
            .iter()
            .filter(|c| c.is_alphanumeric())
            .map(|c| c.to_string())
            .collect();
        let normalized_arg = chars_in_string.join("");

        normalized_vec.push(normalized_arg);
    }

    Ok(normalized_vec)
}
