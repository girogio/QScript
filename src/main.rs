mod lexing;
mod models;
mod utils;

use clap::{command, Parser as ClapParser, Subcommand};
use console::style;
use std::{io::Write, path::PathBuf};
use utils::SimpleBuffer;

use crate::lexing::Lexer;

#[derive(ClapParser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    subcmd: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Runs the PArL lexer on the given file.
    #[clap(name = "lex")]
    Lexer {
        /// The file to lex.
        #[clap(name = "file")]
        in_file: PathBuf,
    },
    #[clap(name = "parse")]
    /// Runs the PArL parser on the given file and prints the AST.
    Parse {
        /// The file to print.
        #[clap(name = "file")]
        in_file: PathBuf,
    },
    /// Runs the PArL formatter on the given file.
    #[clap(name = "fmt")]
    Format {
        /// The file to format.
        #[clap(name = "file")]
        in_file: PathBuf,
    },
    /// Runs the PArL semantic analyzer on the given file.
    #[clap(name = "sem")]
    Semantic {
        /// The PArL source file to analyze.
        #[clap(name = "file")]
        in_file: PathBuf,
    },
    #[clap(name = "compile")]
    /// Compiles the given file to PArIR instructions.
    Compile {
        /// The PArL source file to compile.
        #[clap(name = "file")]
        in_file: PathBuf,
        #[clap(short, long)]
        /// The PArIR output file.
        output: Option<PathBuf>,
    },
}

fn main() {
    let cli = Cli::parse();

    let in_file = match &cli.subcmd {
        Commands::Lexer { in_file } => in_file,
        // Commands::Format { in_file } => in_file,
        // Commands::Semantic { in_file } => in_file,
        // Commands::Parse { in_file } => in_file,
        // Commands::Compile { in_file, .. } => in_file,
        _ => unimplemented!("Command not implemented"),
    };

    if !in_file.exists() {
        let msg = style("error: file not found").red().bold().for_stderr();
        eprintln!("{} `{}`...", msg, style(in_file.display()).cyan());
        std::process::exit(1);
    }

    let input = match std::fs::read_to_string(in_file) {
        Ok(input) => input,
        Err(_) => {
            let msg = style("error: could not read file")
                .red()
                .bold()
                .for_stderr();
            eprintln!("{} `{}`...", msg, style(in_file.display()).cyan());
            std::process::exit(1);
        }
    };

    let mut lexer: Lexer<SimpleBuffer> = Lexer::new(&input, in_file, None);

    let tokens = match lexer.lex() {
        Ok(tokens) => tokens,
        Err(e) => {
            for err in e {
                eprintln!("{}", err);
            }
            std::process::exit(1);
        }
    };

    match &cli.subcmd {
        Commands::Lexer { .. } => {
            println!("{} lexed successfully.", style(in_file.display()).cyan());
            for token in &tokens {
                println!("{:?}", token);
            }
            std::process::exit(0);
        }

        _ => unimplemented!("Command not implemented"),
    }
}
