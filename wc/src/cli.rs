use std::path::PathBuf;
use clap::{Parser, Subcommand};

/*
The Cli struct acts as the entry point and blueprint for your entire command-line application, 
translating raw text typed into the terminal into structured Rust data.

* DOC https://docs.rs/clap/latest/clap/_derive/_tutorial/index.html
*/
#[derive(Parser, Debug)]
#[command(version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Wc {
      #[arg(short = 'l', long="lines")]
      lines: bool,
      file: PathBuf
    }
}
