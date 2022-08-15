use anyhow::{bail, Result};
use clap::Parser;
use ts_cursor::{cursor::STKind::*, file::File, utils::dumper::*};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Arguments {
    /// file to parse
    #[clap(value_parser)]
    file: String,

    /// language to parse (php, js)
    #[clap(value_parser)]
    language: String,

    /// concrete vs abstract
    #[clap(short, long)]
    concrete: bool,
}

fn main() -> Result<()> {
    let args = Arguments::parse();

    let language = match args.language.as_str() {
        "php" => tree_sitter_php::language(),
        "js" | "javascript" => tree_sitter_javascript::language(),
        _ => bail!("unrecognized language {}", args.language),
    };

    let file = File::new(&args.file, language)?;

    let s = match args.concrete {
        true => Dumper::new(vec![&file]).dump(Concrete),
        false => Dumper::new(vec![&file]).dump(Abstract),
    };

    println!("{s}");
    Ok(())
}
