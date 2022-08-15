use anyhow::{bail, Result};
use clap::Parser;
use ts_cursor::{file::File, utils::dumper::*};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Arguments {
    /// file to parse
    #[clap(value_parser)]
    file: String,

    // language to parse
    #[clap(value_parser)]
    language: String,

    /// concrete vs abstract
    #[clap(short, long)]
    concrete: bool,
}

fn main() -> Result<()> {
    let args = Arguments::parse();

    let language = match args.language.as_str() {
        "php" => Some(tree_sitter_php::language()),
        "js" | "javascript" => Some(tree_sitter_javascript::language()),
        _ => None,
    };

    if let None = language {
        bail!("unrecognized language {}", args.language);
    }
    let language = language.unwrap();

    // let source_code = std::fs::read_to_string(args.file.clone())
    //     .expect(&format!("failed to read file {}", args.file));
    let file = File::new(&args.file, language)?;

    let s = Dumper::new(vec![&file]).dump(args.concrete);

    println!("{s}");

    Ok(())
}
