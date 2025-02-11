use std::fs;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: markdown_to_html <input_file.md>");
        return;
    }

    let filename = &args[1];

    match fs::read_to_string(filename) {
        Ok(contents) => {
            println!("📄 Successfully read the file!");
            println!("File Content:\n{}", contents);
        }
        Err(err) => {
            eprintln!("❌ Error reading the file: {}", err);
        }
    }
}
