use std::fs;
use std::env;
use regex::Regex;

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

            // Save the converted HTML to output.html
            let output_filename = "output.html";
            fs::write(output_filename, &html_output).expect("❌ Unable to write file");
            println!("✅ Converted HTML saved as {}", output_filename);
        }
        Err(err) => {
            eprintln!("❌ Error reading the file: {}", err);
        }
    }
}

// 📝 Function to Convert Markdown to HTML
fn markdown_to_html(markdown: &str) -> String {
    let mut html = String::new();
    let bold_regex = Regex::new(r"\*\*(.*?)\*\*").unwrap();
    let italic_regex = Regex::new(r"\*(.*?)\*").unwrap();
    let code_regex = Regex::new(r"`(.*?)`").unwrap();

    for line in markdown.lines() {
        let converted_line = if line.starts_with("# ") {
            format!("<h1>{}</h1>", &line[2..])
        } else if line.starts_with("## ") {
            format!("<h2>{}</h2>", &line[3..])
        } else if line.starts_with("### ") {
            format!("<h3>{}</h3>", &line[4..])
        } else {
            let line = bold_regex.replace_all(line, "<strong>$1</strong>").to_string();
            let line = italic_regex.replace_all(&line, "<em>$1</em>").to_string();
            code_regex.replace_all(&line, "<code>$1</code>").to_string()
        };

        html.push_str(&converted_line);
        html.push('\n');
    }

    html
}
