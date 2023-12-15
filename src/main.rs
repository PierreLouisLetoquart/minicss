use std::fs::read_to_string;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

// TODO: use cli crate to parse args (clap)
// TODO: find solution to use the crate with cli and without cli (e.g. cargo run -- --whatever)
// TODO: separate the code in modules (e.g. css file processing, file writing, etc.) in src/lib.rs
// and use it in main.rs (for error handling, etc.)
fn main() {
    // get args, first arg is the path to the css file, second arg is the path to the target file (optional)
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Veuillez fournir le chemin vers le fichier CSS à minifier.");
        std::process::exit(1);
    }

    let css_file = PathBuf::from(&args[1]);
    let target_path = match args.len() {
        2 => None,
        3 => Some(Path::new(&args[2])),
        _ => {
            eprintln!("Trop d'arguments.");
            std::process::exit(1);
        }
    };

    // check if css file exists
    if !css_file.exists() {
        eprintln!("Le fichier {:?} n'existe pas.", css_file);
        std::process::exit(1);
    }

    // check if css file is a file and get its content
    let file_content = read_to_string(&css_file).unwrap_or_else(|err| {
        eprintln!("Impossible de lire le fichier {:?} : {}", css_file, err);
        std::process::exit(1);
    });

    let new_content = process_file_content(&file_content);

    overwrite_file(&css_file, &new_content, target_path).unwrap_or_else(|err| {
        eprintln!(
            "Impossible d'écrire dans le fichier {:?} : {}",
            css_file, err
        );
        std::process::exit(1);
    });
}

fn process_file_content(file_content: &str) -> String {
    let mut new_content = String::new();
    let mut is_multiline_comment = false;

    for line in file_content.lines() {
        // TODO: look if only trim is enough, the process will be after all if instructions
        let line = process_line(line.trim());

        if line.is_empty() {
            continue;
        }

        if line.starts_with("//") {
            continue;
        }

        if line.starts_with("/*") && !line.ends_with("*/") {
            is_multiline_comment = true;
            continue;
        }

        if line.starts_with("/*") && line.ends_with("*/") {
            continue;
        }

        if is_multiline_comment {
            if line.ends_with("*/") {
                is_multiline_comment = false;
            }
            continue;
        }

        // TODO: process line content (remove spaces, shorten colors, etc.)

        new_content.push_str(&line);
    }

    new_content.replace(";}", "}")
}

fn process_line(line: &str) -> String {
    // TODO: better processing, meaning:
    //  - remove spaces before and after the line str 
    //  - try shortening the colors (e.g. #ffffff -> #fff)
    //  - try shertening the properties (e.g. margin: 0px -> margin: 0)
    //  - verify if the line contains a comment and remove it
    //  - not shorten spaces between properties without coma (e.g. margin: 0px 0px 0px 0px -> margin: 0 0 0 0)

    // remove spaces inside the line str
    line.replace(" ", "")
}

// TODO: rewrite the function in a more functional + reliable way
// TODO: add a test for the overwrite_file function
fn overwrite_file(
    file_path: &PathBuf,
    new_content: &str,
    target_path: Option<&Path>,
) -> Result<(), io::Error> {
    let target_path = match target_path {
        Some(path) => path.to_path_buf(),
        None => file_path.clone(),
    };

    let mut file = std::fs::File::create(&target_path)?;

    file.write_all(new_content.as_bytes())?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_line() {
        let line = "  .class {  color: red;  }  ";
        let expected = ".class{color:red;}";
        let result = process_line(line);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_process_file_content() {
        let file_content = r#"
            * {
                margin: 0;
                padding: 0;
            }

            .class {
                color: red;
            }

            /* comment */ 
            .class2 {
                color: blue;
            }

            /*
             * multiline comment
             */ 
            .class3 {
                color: green;
                font: 12px;
            }
            "#;
        let expected = r#"*{margin:0;padding:0}.class{color:red}.class2{color:blue}.class3{color:green;font:12px}"#;
        let result = process_file_content(file_content);
        assert_eq!(result, expected);
    }

    // TODO: add tests for color shortening
    // TODO: add tests for property shortening
    // TODO: add tests for comment removal
    // TODO: add tests for space removal
    // TODO: add tests for semicolon removal
    // TODO: add tests for space between properties without coma
    // TODO: add tests for overwrite_file
    //
    // TODO: add tests for the cli (E2E tests)
}
