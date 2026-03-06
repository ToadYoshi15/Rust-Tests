
use std::env;
use std::fs;
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: cool <command> [args]");
        return;
    }
    let command = &args[1];
    let res: Result<String, std::io::Error> = fs::read_to_string(command);
    if res.is_ok() {
        println!("Command output: \n---------\n {}", res.unwrap());
    } else {
        eprintln!("Error: {}", res.err().unwrap());
    }
}
//TODO: FIX error handling of fake files