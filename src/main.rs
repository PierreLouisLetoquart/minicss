use clap::Parser;
use minicss::minify_content;
use minicss::source_exists;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    source: String,

    #[arg(short, long)]
    target: String,
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
