mod interpreter;
mod compile;

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

    let mut input = std::io::stdin();
    let mut output = std::io::stdout();

    interpreter::run(program, &mut input, &mut output).unwrap_or_else(|err| {
        eprintln!("Error running program: {}", err);
        std::process::exit(1);
    });
}