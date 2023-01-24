use std::io::BufRead;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::{env, fs, process};

use chrono::{DateTime, Local};
use syntect::easy::HighlightFile;
use syntect::highlighting::{Style, ThemeSet};
use syntect::parsing::SyntaxSet;
use syntect::util::as_24_bit_terminal_escaped;

mod formatter;

pub struct App {
    pub file_path: String,
    pub file_content: String,
    pub highlighted_lines: Vec<String>,
}

impl App {
    pub fn new(args: &[String]) -> Result<App, &'static str> {
        if args.len() < 2 {
            return Err("Not enough arguments");
        }

        let file_path = args[1].to_string();

        Ok(App {
            file_path,
            file_content: String::from(""),
            highlighted_lines: Vec::new(),
        })
    }

    pub fn set_file_content(&mut self) {
        let file_content_result = fs::read_to_string(&self.file_path.to_string());

        match file_content_result {
            Ok(content) => self.file_content = content.to_string(),
            Err(error) => {
                println!("Error reading file: {error}");
                process::exit(1);
            }
        };
    }

    pub fn get_highlighted_lines(&mut self) {
        let syntax_set = SyntaxSet::load_defaults_newlines();
        let theme_set = ThemeSet::load_defaults();

        let mut highlighter = HighlightFile::new(
            &self.file_path,
            &syntax_set,
            &theme_set.themes["base16-ocean.dark"],
        )
        .unwrap();

        let mut line = String::new();

        while highlighter.reader.read_line(&mut line).unwrap() > 0 {
            {
                let regions: Vec<(Style, &str)> = highlighter
                    .highlight_lines
                    .highlight_line(&line, &syntax_set)
                    .unwrap();

                self.highlighted_lines
                    .push(as_24_bit_terminal_escaped(&regions[..], false));
            }

            line.clear();
        }
    }

    pub fn print_file_details(&self) {
        let metadata_result = fs::metadata(&self.file_path);

        match metadata_result {
            Ok(meta) => {
                let filename = Path::new(&self.file_path)
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap();
                let size = formatter::format_size(meta.len());
                let modified: DateTime<Local> = DateTime::from(meta.modified().unwrap());
                let permissions = formatter::parse_permissions(meta.permissions().mode() as u16);

                println!("########## File: {} ##########", filename);
                println!("Size: {}", size);
                println!("Permissions: {}", permissions);
                println!("Date Modified: {}", modified.format("%D %H:%M").to_string());
                println!("################################\n");
            }
            Err(error) => {
                println!("Error getting file metadata: {error}");
            }
        }
    }

    pub fn print_file_content(&self) {
        for line in &self.highlighted_lines {
            print!("{line}");
        }
    }
}

pub fn run() {
    let args: Vec<String> = env::args().collect();
    let mut app = App::new(&args).unwrap_or_else(|err| {
        println!("Error parsing arguments: {err}");
        process::exit(1);
    });

    app.set_file_content();
    app.get_highlighted_lines();
    app.print_file_details();
    app.print_file_content();
}
