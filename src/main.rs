use std::io;
use std::path::PathBuf;
use clap::Parser;

mod util;


#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// The XML files to use
    #[clap(required = true)]
    nessus: Vec<PathBuf>,

    /// Outputs in CSV format
    #[clap(short, long, conflicts_with="json")]
    csv: bool,

    /// Outputs in CSV format
    #[clap(short, long, conflicts_with="csv")]
    json: bool,

    /// Specifies the base name of the output file. The correct extension will be appended based on the selected format.
    #[clap(short, long)]
    output: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    util::warn("Use with caution. You are responsible for your actions.".to_string());

    let writer: Box<dyn io::Write> = match cli.output {
        Some(path) => Box::new(
            std::fs::File::create(path).expect("An error occured with the output file")
        ), // If a path is specified, create and use a file writer
        None => Box::new(io::stdout()), // If no path is specified, default to stdout
    };

    if cli.json {
        nessusx::as_json(cli.nessus, writer);
    } else {
        nessusx::as_csv(cli.nessus, writer);
    }
    
    Ok(())
}