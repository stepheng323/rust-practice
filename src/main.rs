use std::fs;
use std::io::Error;

fn main() -> Result<(), Error>{

    // let text = fs::read_to_string("logs.txt").expect("failed to read log file");

    // let error_logs = extract_errors(text.as_str());

    // fs::write("error.txt", error_logs.join("\n")).expect("failed to write error to file")


    let text = fs::read_to_string("logs.txt")?;

    let error_logs = extract_errors(text.as_str());

    fs::write("error.txt", error_logs.join("\n"))?;

    Ok(())

    // match fs::read_to_string("logs.txt") {
    //     Ok(value) => {
    //         let error_logs = extract_errors(value.as_str());
    //         match fs::write("error.txt", error_logs.join("\n")) {
    //             Ok(..)=> println!("error written successfully"),
    //             Err(err) => println!("Error writing text to file {}", err)
    //         }
    //     }
    //     Err(error) => {
    //         println!("An error occurred {}", error);
    //     }
    // }
}
fn extract_errors(text: &str) -> Vec<String> {
    let split_text = text.split("\n");
    let mut result = vec![];
    for line in split_text {
        if line.starts_with("ERROR") {
            result.push(line.to_string());
        }
    }
    result
}
