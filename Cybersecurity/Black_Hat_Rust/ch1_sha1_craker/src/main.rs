use clap::{arg, command, value_parser, ArgAction, Command};
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("Usage:");
        println!("sh1_cracker: <wordlist.txt> <sha1_hash>");
        return;

    }
}
