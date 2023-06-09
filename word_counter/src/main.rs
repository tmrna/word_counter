use std::{env, fs::read_to_string, process::exit};

mod word_counter;
use word_counter::WordCounter;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("No file was provided");
        exit(1);
    }
    let file_path = &args[1];
    let contents = read_to_string(file_path).expect("Could not open file");
    //let mut counter = WordCounter::default();
    //counter.insert(&contents);
    //counter.print();
}
