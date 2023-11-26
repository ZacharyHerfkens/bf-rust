mod compile;
mod interpret;
mod tests;

use clap::Parser;


#[derive(Parser, Debug)]
struct Cli {
    file: std::path::PathBuf,
}

fn main() {
    let clargs = Cli::parse();
    let program = std::fs::read_to_string(clargs.file)
        .unwrap_or_else(|err| {
            eprintln!("Error reading program file: {}", err);
            std::process::exit(1);
        });  

    let input = std::io::stdin();
    let output = std::io::stdout();

    interpret::run(&program, input, output).unwrap_or_else(|err| {
        eprintln!("Error running program: {}", err);
        std::process::exit(1);
    });
}