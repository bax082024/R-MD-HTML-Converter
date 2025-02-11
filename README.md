# Markdown to HTML Converter

A Markdown to HTML converter written in Rust. This command-line tool reads a Markdown (.md) file, converts it into a properly formatted HTML file, and applies basic styling for a clean presentation.

---

## Features

- Convert Markdown to HTML.
- Supports headers, lists, code blocks, links and tables.
- Syntax highlighting for code blocks.
- CSS-styled output for better readability.
- Easy to use command line interface.

--- 

## Installation

**Prerequisites**

- Rust (Install via **rustup.rs**)

**Clone & Build**

1. Clone repository :
    - https://github.com/bax082024/R-MD-HTML-Converter.git

2. **Build the Project**
    - cargo build --release

---

## How to use

1. Create a `.md` file. (example `Test.md`)

2. Write the code in the file `Test.md`.

3. run the program and create the `output.html` file.
    - **cargo run Test.md**

4. Open the `output.html` file in any browser, or go live function in IDE.

--- 

## Supported Markdown Elements

- Headers (# H1, ## H2, ### H3, etc.)
- Bold (**bold**) & Italics (*italic*)
- Inline Code (\code``)
- Code Blocks (```rust ... ```)
- Blockquotes (> Quote)
- Lists (Ordered & Unordered)
- Tables
- Horizontal Rules (--- or ***)
- Images (![Alt Text](url))
- Links ([Text](URL))

---

