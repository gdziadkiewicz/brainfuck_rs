use std::{error::Error, fs, path::PathBuf};

use brainfuck_rs::brain_luck;
use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to file with Brainf**k code
    #[arg(short, long)]
    code: PathBuf,

    /// Path to file with the input for the program
    #[arg(short, long)]
    input: Option<PathBuf>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let code = fs::read_to_string(&args.code)?;
    let input = args.input.as_ref().map(fs::read).unwrap_or(Ok(vec![]))?;
    let output = brain_luck(&code, input);
    println!("{}", String::from_utf8(output?)?);
    Ok(())
}
