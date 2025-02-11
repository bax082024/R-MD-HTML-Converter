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
            let html_output = markdown_to_html(&contents);
            println!("Converted HTML:\n{}", html_output);
        }
        Err(err) => {
            eprintln!("Error reading the file: {}", err);
        }
    }

    fn markdown_to_html(markdown: &str) -> String {
        let mut html = String::new();
    
        for line in markdown.lines() {
            let converted_line = if line.starts_with("# ") {
                format!("<h1>{}</h1>", &line[2..])
            } else if line.starts_with("## ") {
                format!("<h2>{}</h2>", &line[3..])
            } else if line.starts_with("### ") {
                format!("<h3>{}</h3>", &line[4..])
            } else {
                line.to_string()
            };
    
            html.push_str(&converted_line);
            html.push('\n');
        }
    
        html
    }
}
