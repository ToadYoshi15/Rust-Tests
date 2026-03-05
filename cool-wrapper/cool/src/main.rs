
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

    println!("Command output: \n---------\n {}", res.to_string());
}
//TODO: FIX error handling of fake files