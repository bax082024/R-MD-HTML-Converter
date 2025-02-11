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

            let output_filename = "output.html";
            fs::write(output_filename, &html_output).expect("❌ Unable to write file");
            println!("✅ Converted HTML saved as {}", output_filename);
        }
        Err(err) => {
            eprintln!("❌ Error reading the file: {}", err);
        }
    }
}

fn markdown_to_html(markdown: &str) -> String {
    let mut html = String::new();
    let mut in_list = false;
    let mut in_ordered_list = false;

    let bold_regex = Regex::new(r"\*\*(.*?)\*\*").unwrap();
    let italic_regex = Regex::new(r"\*(.*?)\*").unwrap();
    let code_regex = Regex::new(r"`(.*?)`").unwrap();
    let link_regex = Regex::new(r"\[(.*?)\]\((.*?)\)").unwrap();
    let ordered_list_regex = Regex::new(r"^\d+\.\s(.*)").unwrap();

    for line in markdown.lines() {
        let converted_line = if line.starts_with("# ") {
            format!("<h1>{}</h1>", &line[2..])
        } else if line.starts_with("## ") {
            format!("<h2>{}</h2>", &line[3..])
        } else if line.starts_with("### ") {
            format!("<h3>{}</h3>", &line[4..])
        } else if line.starts_with("> ") {
            format!("<blockquote>{}</blockquote>", &line[2..])
        } else if ordered_list_regex.is_match(line) {
            if !in_ordered_list {
                html.push_str("<ol>\n");
                in_ordered_list = true;
            }
            format!("<li>{}</li>", &ordered_list_regex.captures(line).unwrap()[1])
        } else if line.starts_with("- ") {
            if !in_list {
                html.push_str("<ul>\n");
                in_list = true;
            }
            format!("<li>{}</li>", &line[2..]) // ✅ Fix list handling
        } else if line.trim() == "---" || line.trim() == "***" { // ✅ Fixed Horizontal Rule
            "<hr>".to_string() // ✅ Correctly return <hr> instead of doing nothing
        } else {
            if in_list {
                html.push_str("</ul>\n");
                in_list = false;
            }
            if in_ordered_list {
                html.push_str("</ol>\n");
                in_ordered_list = false;
            }
            
            let line = bold_regex.replace_all(line, "<strong>$1</strong>").to_string();
            let line = italic_regex.replace_all(&line, "<em>$1</em>").to_string();
            let line = code_regex.replace_all(&line, "<code>$1</code>").to_string();
            let line = link_regex.replace_all(&line, "<a href=\"$2\">$1</a>").to_string();
            format!("<p>{}</p>", line)
        };

        html.push_str(&converted_line);
        html.push('\n');
    }

    if in_list {
        html.push_str("</ul>\n");
    }
    if in_ordered_list {
        html.push_str("</ol>\n");
    }

    html
}




