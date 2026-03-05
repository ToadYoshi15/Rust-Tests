
use std::env;
use std::fs;
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: cool <command> [args]");
        return;
    }
    let command = &args[1];
    let res: String = fs::read_to_string(command).expect("Failed to read command file");
    println!("Command output: \n---------\n {}", res);
}
//TODO: FIX