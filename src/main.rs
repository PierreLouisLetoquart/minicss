use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    source: String,

    #[arg(short, long)]
    target: String,
}

fn source_exists(source: &str) -> bool {
    std::path::Path::new(source).exists()
}

fn is_unnecessary(prev: char, c: char) -> bool {
    let unnecessary_combinations: std::collections::HashSet<(char, char)> = [
        (';', ' '),
        ('{', ' '),
        ('}', ' '),
        ('(', ' '),
        (')', ' '),
        (',', ' '),
        (':', ' '),
        ('=', ' '),
        ('+', ' '),
        ('-', ' '),
        ('*', ' '),
        ('/', ' '),
        ('%', ' '),
        ('!', ' '),
        ('>', ' '),
        ('<', ' '),
        ('&', ' '),
        ('|', ' '),
        ('^', ' '),
        ('~', ' '),
        ('[', ' '),
        (']', ' '),
        (' ', ' '),
    ]
    .iter()
    .cloned()
    .collect();

    unnecessary_combinations.contains(&(prev, c))
}

fn minify_content(content: &str) -> String {
    let mut minified = Vec::new();
    let mut chars = content.chars();
    let mut prev = ' ';

    while let Some(c) = chars.next() {
        if c == '\n' {
            continue;
        }

        if c == ' ' && is_unnecessary(prev, c) {
            continue;
        }

        if c == '/' {
            if let Some('*') = chars.next() {
                while let Some(c) = chars.next() {
                    if c == '*' {
                        if let Some('/') = chars.next() {
                            break;
                        }
                    }
                }
                continue;
            }
        }

        minified.push(c);
        prev = c;
    }

    minified.into_iter().collect()
}

fn main() {
    let args = Args::parse();

    println!("Minifying {} to {}", args.source, args.target);

    if !source_exists(&args.source) {
        eprintln!("Source file does not exist");
        std::process::exit(1);
    }

    let content = std::fs::read_to_string(&args.source).expect("Could not read source file");

    let minified_content = minify_content(&content);

    // Additional replacements
    let final_content = minified_content.replace(";}", "}").replace(" {", "{");

    std::fs::write(&args.target, final_content).expect("Could not write to target file");

    println!("DONE!");
}
