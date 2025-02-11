use std::fs;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: markdown_to_html <input_file.md>");
        return;
    }

    

}