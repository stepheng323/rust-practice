use std::fs;

fn main() {
    let mut error_logs = vec![];
    match fs::read_to_string("logs.txt") {
        Ok(value) => {
            error_logs = extract_errors(value.as_str());
        }
        Err(error) => {
            println!("An error occurred {}", error);
        }
    }
    println!("{:#?}", error_logs)
}


fn extract_errors(text: &str) -> Vec<&str> {
    let split_text = text.split("\n");
    let mut result = vec![];
    for line in split_text {
        if line.starts_with("ERROR") {
            result.push(line);
        }
    }
    result
}
