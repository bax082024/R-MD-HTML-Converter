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
            let styled_output = add_css_and_js(&html_output);

            let output_filename = "output.html";
            fs::write(output_filename, &styled_output).expect("Unable to write file");
            println!("Converted HTML saved as {}", output_filename);
        }
        Err(err) => {
            eprintln!("Error reading the file: {}", err);
        }
    }
}

fn markdown_to_html(markdown: &str) -> String {
    let mut html = String::new();
    let mut in_code_block = false;
    let mut in_table = false;
    let mut is_header = false;

    let bold_regex = Regex::new(r"\*\*(.*?)\*\*").unwrap();
    let italic_regex = Regex::new(r"\*(.*?)\*").unwrap();
    let code_regex = Regex::new(r"`(.*?)`").unwrap();
    let link_regex = Regex::new(r"\[(.*?)\]\((.*?)\)").unwrap();
    let image_regex = Regex::new(r"!\[(.*?)\]\((.*?)\)").unwrap();
    let table_regex = Regex::new(r"^\s*\|.*\|$").unwrap();
    let table_separator_regex = Regex::new(r"^\s*\|[-\s]+\|$").unwrap();
    let header_regex = Regex::new(r"^(#{1,6})\s+(.*)").unwrap();

    for line in markdown.lines() {
        if line.contains("<![CDATA[") || line.contains("WebSocket") || line.contains("Live Server") {
            continue;
        }

        let converted_line = if header_regex.is_match(line) {
            let header = header_regex.captures(line).unwrap();
            let level = header[1].len();
            let content = &header[2];
            format!("<h{}>{}</h{}>", level, content, level)
        } else if line.starts_with("```") {
            if in_code_block {
                in_code_block = false;
                format!("</code></pre>")
            } else {
                in_code_block = true;
                let code_lang = line.trim_start_matches("```").to_string();
                format!("<pre><code class=\"language-{}\">", code_lang)
            }
        } else if in_code_block {
            format!("{}", line)
        } else if table_regex.is_match(line) {
            if !in_table {
                html.push_str("<table>\n");
                in_table = true;
                is_header = true;
            }

            if table_separator_regex.is_match(line) {
                is_header = false;
                continue;
            }

            let row = line.trim().trim_start_matches('|').trim_end_matches('|');
            let cells: Vec<String> = row.split('|').map(|c| {
                if is_header {
                    format!("<th>{}</th>", c.trim())
                } else {
                    format!("<td>{}</td>", c.trim())
                }
            }).collect();
            let row_html = format!("<tr>{}</tr>", cells.join(""));
            is_header = false;
            row_html
        } else {
            let line = bold_regex.replace_all(line, "<strong>$1</strong>").to_string();
            let line = italic_regex.replace_all(&line, "<em>$1</em>").to_string();
            let line = code_regex.replace_all(&line, "<code>$1</code>").to_string();
            let line = link_regex.replace_all(&line, "<a href=\"$2\">$1</a>").to_string();
            let line = image_regex.replace_all(&line, "<img src=\"$2\" alt=\"$1\">").to_string();
            format!("<p>{}</p>", line)
        };

        html.push_str(&converted_line);
        html.push('\n');
    }

    if in_table {
        html.push_str("</table>\n");
    }

    html
}


fn add_css_and_js(html: &str) -> String {
    format!(
        "<!DOCTYPE html>
        <html>
        <head>
        <link rel=\"stylesheet\" href=\"https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.5.1/styles/default.min.css\">
        <script src=\"https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.5.1/highlight.min.js\"></script>
        <script>hljs.highlightAll();</script>
        <style>
            body {{
                font-family: Arial, sans-serif;
                line-height: 1.6;
                max-width: 800px;
                margin: 20px auto;
                padding: 20px;
            }}
            h1, h2, h3 {{
                margin-top: 20px;
                border-bottom: 2px solid #ddd;
                padding-bottom: 5px;
            }}
            pre {{
                background: #f4f4f4;
                padding: 10px;
                border-radius: 5px;
                overflow-x: auto;
            }}
            code {{
                font-family: monospace;
                background: #eee;
                padding: 2px 5px;
                border-radius: 3px;
            }}
            table {{
                width: 100%;
                border-collapse: collapse;
                margin-top: 10px;
            }}
            th, td {{
                border: 1px solid #ddd;
                padding: 8px;
                text-align: left;
            }}
            th {{
                background: #f8f8f8;
            }}
            tr:nth-child(even) {{
                background-color: #f2f2f2;
            }}
            ul, ol {{
                padding-left: 20px;
            }}
            li {{
                margin: 5px 0;
            }}
        </style>
        </head>
        <body>{}</body>
        </html>",
        html
    )
}
