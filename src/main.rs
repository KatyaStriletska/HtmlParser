use clap::{Parser, Subcommand}; // Використовуємо тільки clap для CLI
use html_parser::{parse_html, HtmlElem};
use std::fs::read_to_string;

#[derive(Parser)]
#[command(
    name = "HTML Parser CLI Tool",
    about = "A tool to parse HTML files and display their DOM structure.",
    long_about = None,
    after_help = "Example:\n cargo run -- parse path/to/your_file.html\n\nAuthor:\n  Striletska Kateryna"
)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Parse { path: String },
    Credits,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    match args.command {
        Commands::Parse { path } => parse(&path),
        Commands::Credits => print_credits(),
    }
}

fn parse(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", path);
    let html = read_to_string(path)?;
    println!("{}", html);

    let parsed_html = parse_html(html.as_str())?;
    for item in parsed_html {
        print_tree(&item, "");
    }
    Ok(())
}

fn print_credits() -> Result<(), Box<dyn std::error::Error>> {
    println!("HTML PARSER PROJECT");
    println!("Author : Striletska Kateryna");
    println!("Version: 1.0.0");
    Ok(())
}

fn print_tree(root: &HtmlElem, indent: &str) {
    match root {
        HtmlElem::Tag { tag_name, children } => {
            println!("{} {}", indent, tag_name);
            if !children.is_empty() {
                let new_indent = format!("{}  ", indent);
                for child in children {
                    print_tree(child, &new_indent);
                }
            }
        }
        HtmlElem::Text(text) => {
            println!("{} ({})", indent, text);
        }
        HtmlElem::Documentation(doctype) => {
            println!("{}", doctype);
        }
    }
}
