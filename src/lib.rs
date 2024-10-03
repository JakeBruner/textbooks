
use std::fs::File;
use opener::open;
use open as browser_open;
use std::io::Error;
use colored::Colorize;


const TEXTBOOKS_PATH: &'static str = "/Users/jakebruner/Documents/textbooks/";

pub fn get_sub_directories(path: &str) -> Result<Vec<String>, TextbookError> {
    Ok(std::fs::read_dir(path)?
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| {
            let path = entry.path();
            if path.is_dir() {
                path.file_name()
                    .and_then(|name| name.to_str())
                    .map(|name| name.to_owned())
            } else {
                None
            }
        })
        .collect::<Vec<String>>())
}

pub fn pretty_string_list(list: &[String]) -> String {
    match list.len() {
        0 => String::new(),
        1 => list[0].clone(),
        _ => {
            let last = list.last().unwrap().clone();
            let first_parts = list.iter().take(list.len() - 1).map(String::as_str).collect::<Vec<&str>>().join(", ");
            format!("{}, and {}", first_parts, last)
        }
    }
}

pub fn run(config: Config) -> Result<(), TextbookError> {
    // navigate to TEXTBOOKS_PATH/class directory, search all files, if solutions is true, open the only file without solutions, if solutions is false, open the only file with solutions
    // if there are multiple files, print them out and ask the user to specify which one they want to open
    // if there are no files, print an error message

    let mut path = TEXTBOOKS_PATH.to_string();
    let sub_directories = get_sub_directories(&path)?;
    println!("{} {}{}", "Found textbooks for:".bright_blue(), pretty_string_list(&sub_directories).italic().bright_blue(), ".".bright_blue());

    path.push_str(config.class);
    path.push_str("/");
    let dir = std::fs::read_dir(path).map_err(|_| TextbookError::new("Could not find this class's textbook folder. Make sure it is located in ~/Documents/textbooks/[class]/."));
        //.map_err(|_|TextbookError::new("Could not find this class's textbook folder. Make sure it is located in ~/Documents/textbooks/[class]/"))?;
    let mut files: Vec<String> = Vec::new();
    for entry in dir? {
        let entry = entry?;
        let path = entry.path();
        let path = path.to_str().ok_or(TextbookError::new("Failed to convert path to string."))?;
        files.push(path.to_string());
    }

    if files.len() == 0 {
        eprintln!("No files found for class {}", config
            .class);
        std::process::exit(1);
    }
    return if config.solutions {
        let file = files.iter().find(|&f| f.to_lowercase().contains("solutions")).ok_or(TextbookError::new("No solutions manual found."))?;
        handle_open(file, config.inbrowser);
        println!("{} {} {} {}{}","Opening".bold().green(), "solutions".bold().underline().bright_green(), "manual for class".bold().green(), config.class.bold().bright_green().underline(), "...".bold().green());
        Ok(())
    } else {
        let file = files.iter().find(|&f| !f.contains("solutions")).ok_or(TextbookError::new("No textbook found."))?;
        handle_open(file, config.inbrowser);
        println!("{} {}{}", "Opening textbook for class".bold().green(), config.class.bold().bright_green().underline(), "...".bold().green());
        Ok(())
    };
}

#[derive(Debug)]
pub struct TextbookError {
    details: String,
}
impl TextbookError {
    fn new(msg: &str) -> TextbookError {
        TextbookError { details: msg.to_string() }
    }
}
impl std::fmt::Display for TextbookError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.details)
    }
}
impl std::error::Error for TextbookError {
    fn description(&self) -> &str {
        &self.details
    }
}
impl From<Error> for TextbookError {
    fn from(err: Error) -> TextbookError {
        TextbookError::new(&err.to_string())
    }

}

pub struct Config<'a> {
    pub class: &'a str,
    pub solutions: bool,
    pub inbrowser: bool,
}

impl Config<'_> {
    pub fn new(args: &[String]) -> Result<Config, TextbookError> {
        if args.len() < 2 {
            return Err(TextbookError::new("Please supply the name of a class. Type -h for help."));
        }

        let mut solutions = false;
        let mut inbrowser = false;
        let mut class = "";

        let mut i = 1;
        while i < args.len() {
            match args[i].as_str() {
                "-h" => {
                    println!("{}", "Usage: textbooks [options] <class>".bold());
                    println!("Options:");
                    println!("-h: Display this help message.");
                    println!("-s: Open the solutions manual.");
                    println!("-b: Use bold text.");
                    println!("Open the textbook or solution manual for a given class.");
                    println!("Example: textbooks ece250");
                    println!("Example: textbooks -s -b ece250");
                    std::process::exit(0);
                }
                "-s" => solutions = true,
                "-b" => inbrowser = true,
                "-init" => {
                    File::create(TEXTBOOKS_PATH).expect("Failed to create directory.");
                    i += 1;
                    continue;
                }
                _ if !args[i].starts_with('-') => {
                    class = &args[i];
                    break;
                }
                _ => return Err(TextbookError::new("Invalid option. Type -h for help.")),
            }
            i += 1;
        }

        if class.is_empty() {
            return Err(TextbookError::new("Please supply the name of a class. Type -h for help."));
        }

        Ok(Config { class, solutions, inbrowser })
    }
}

pub fn handle_open(path: &str, inbrowser: bool) -> () {
    if inbrowser {
        browser_open::with(path, "firefox").unwrap_or_else(|err| {
            eprintln!("Problem opening file: {}", err);
            std::process::exit(1);
        });
    } else {
        open(path).unwrap_or_else(|err| {
            eprintln!("Problem opening file: {}", err);
            std::process::exit(1);
        });
    }

}