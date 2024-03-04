use std::io;
use std::io::Read;
use std::fs::File;
use std::path::PathBuf;
use clap::Parser;
use serde_yaml;

mod util;


#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// The XML files to use
    #[clap(required = true)]
    nessus: Vec<String>,

    /// Outputs in CSV format to the specified file, or defaults if not specified
    #[clap(short, long, value_name = "FILE")]
    csv: Option<PathBuf>,

    #[clap(long, value_name = "CONFIG FILE")]
    config: Option<PathBuf>,
}

fn print_prg_info() {
    let prg_info = format!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    let prg_authors = format!("(c) 2023 by {}", env!("CARGO_PKG_AUTHORS"));
    let prg_description = format!("{}", env!("CARGO_PKG_DESCRIPTION"));
    println!("{} {}", prg_info, prg_authors);
    println!("{}", prg_description);
    println!();
}

fn load_config_from_file(path: PathBuf) -> Result<nessusx::Config, Box<dyn std::error::Error>> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let config: nessusx::Config = serde_yaml::from_str(&contents)?;
    Ok(config)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    // Load config if specified, otherwise use default config
    let config = if let Some(config_path) = cli.config {
        load_config_from_file(config_path)?
    } else {
        Default::default()
    };

    util::warn("Use with caution. You are responsible for your actions.".to_string());
    util::warn("Developers assume no liability and are not responsible for any misuse or damage.".to_string());
    
    let writer: Box<dyn io::Write> = match cli.csv {
        Some(path) => Box::new(std::fs::File::create(path).expect("File should not exist")), // If a path is specified, create and use a file writer
        None => Box::new(io::stdout()), // If no path is specified, default to stdout
    };

    nessusx::multiple_to(cli.nessus, writer);
    Ok(())
}