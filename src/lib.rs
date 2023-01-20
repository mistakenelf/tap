use syntect::easy::HighlightFile;
use syntect::highlighting::{Style, ThemeSet};
use syntect::parsing::SyntaxSet;

use std::error::Error;
use std::{env, fs, process};

pub struct App {
    pub file_path: String,
    pub syntax_set: SyntaxSet,
    pub theme_set: ThemeSet,
    pub file_content: String,
}

impl App {
    pub fn new(args: &[String]) -> Result<App, &'static str> {
        if args.len() < 2 {
            return Err("Not enough arguments");
        }

        let file_path = args[1].clone();
        let syntax_set = SyntaxSet::load_defaults_newlines();
        let theme_set = ThemeSet::load_defaults();

        Ok(App {
            file_path,
            syntax_set,
            theme_set,
            file_content: "".to_string(),
        })
    }

    pub fn set_file_content(&mut self) -> Result<(), Box<dyn Error>> {
        let file_content = fs::read_to_string(&self.file_path.to_string())?;

        self.file_content = file_content;

        Ok(())
    }

    pub fn print_contents(&self) {
        let file_content = &self.file_content.to_string();
        println!("{file_content}");
    }
}

pub fn run() {
    let args: Vec<String> = env::args().collect();
    let mut app = App::new(&args).unwrap_or_else(|err| {
        println!("Error parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = app.set_file_content() {
        println!("Error reading file content: {e}");
        process::exit(1);
    }

    app.print_contents();
}
