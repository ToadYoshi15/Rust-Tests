
use std::env;
use std::fs;
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: cool <file> [text]");
        return;
    }
    let command = &args[1];
    if args[2].is_empty() {
        let res: Result<String, std::io::Error> = fs::read_to_string(command);
        if res.is_ok() {
                println!("Command output: \n---------\n {}", res.unwrap());
            } else {
                eprintln!("Error: {}", res.err().unwrap());
        }
    }
    else {
        let text = &args[2];
        let res: Result<(), std::io::Error> = fs::write(command, text);
        if res.is_ok() {
            eprintln!("File written successfully.");
        } else {
            eprintln!("Error: {}", res.err().unwrap());
        }
    }
    
}
