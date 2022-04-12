use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader};


fn get_title() -> String {
    let mut title = String::from(env!("CARGO_PKG_NAME"));
    title.push_str(" (v");
    let version = String::from(env!("CARGO_PKG_VERSION"));
    title.push_str(&version);
    title.push_str("), ");
    let description = String::from(env!("CARGO_PKG_DESCRIPTION"));
    title.push_str(&description);
    title.push_str(".");
    return title;
}

fn parse_markdown_file(filename: &str) {
    print_short_banner();
    println!("[INFO] Trying to parse {}", filename);
    // create a path var from the filename
    let input_filename = Path::new(filename);

    // try to open file
    let file = File::open(&input_filename)
        .expect("[ERROR] Failed to open file!");

    let mut _ptag: bool = false;
    let mut _htag: bool = false;

    // create a place to store all our tokens
    let mut tokens: Vec<String> = Vec::new();

    // read the file line by line
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line_contents = line.unwrap();
        let mut first_char: Vec<char> = line_contents.chars().take(1).collect();
        let mut output_line = String::new();

        match first_char.pop() {
            Some('#') => {
                if _ptag {
                    _ptag = false;
                    output_line.push_str("</p>\n");
                }

                if _htag {
                    _htag = false;
                    output_line.push_str("</h1>\n");
                }

                _htag = true;
                output_line.push_str("\n\n<h1>");
                output_line.push_str(&line_contents[2..]);
            },
            _ => {
                if !_ptag {
                    _ptag = true;
                    output_line.push_str("\n<p>");
                }
                output_line.push_str(&line_contents);
            }
        }

        if _ptag {
            _ptag = false;
            output_line.push_str("</p>\n");
        }

        if _htag {
            _htag = false;
            output_line.push_str("</h1>\n");
        }

        if output_line != "<p></p>\n" {
            tokens.push(output_line);
        }
    }

    for token in &tokens {
        println!("{}", token);
    }
}

fn print_short_banner() {
    println!("{}", get_title());
}

fn print_long_banner() {
    print_short_banner();
    println!("Written by {}", String::from(env!("CARGO_PKG_AUTHORS")));
    let usage = String::from("tinymd <somefile>.md"); println!("Usage: {}", usage); } 

fn usage() {
    print_long_banner();
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    match args.len() {
        2 => parse_markdown_file(&args[1]),
        _ => {
            println!("[ERROR] You forgot to specify the markdown file to parse!");
            usage();
        }
    }
}

