use std::env;

mod util;

fn print_prg_info() {
    let prg_info = format!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    let prg_authors = format!("(c) 2023 by {}", env!("CARGO_PKG_AUTHORS"));
    let prg_description = format!("{}", env!("CARGO_PKG_DESCRIPTION"));
    println!("{} {}", prg_info, prg_authors);
    println!("{}", prg_description);
    println!();
}


fn print_help() {
    print_prg_info();
    println!("Usage: nessusx [options]");
    println!("Options:");
    println!("  -h, --help\t\t\tPrint this help");
    println!("  -v, --version\t\t\tPrint version information");
    println!("  -x, --xml\t\t\tQualys xml to parse");
    println!();
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let mut iter = args.iter();

    let mut is_xml = false;
    let mut xml: &str = "";

    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "-h" | "--help" => {
                print_help();
                std::process::exit(0);
            }
            "-v" | "--version" => {
                print_prg_info();
                std::process::exit(0);
            }
            "-x" | "--xml" => {
                is_xml = true;
                xml = iter.next().unwrap();
            }
            _ => {}
        }
    };

    if !is_xml {
        util::error("-x --xml is a mandatory parameter".to_string());
        std::process::exit(1);        

    }

    util::warn("Use with caution. You are responsible for your actions.".to_string());
    util::warn("Developers assume no liability and are not responsible for any misuse or damage.".to_string());
    
    
    let scan_res = nessusx::from_file(xml);

    match scan_res {
        Ok(scan) => {
            let j = serde_json::to_string_pretty(&scan).unwrap();
            println!("{}", j);
        }

        Err(err) => {
            util::error(format!("{}", err));
        }      
    }
    
}