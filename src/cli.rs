use clap::{Args, Parser, Subcommand};

use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "pdf-generator",
    about = "Generate ATS compatible resumes from JSON"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Start an HTTP server that receives JSON data and replies with a PDF file
    Serve,
    /// Compile a JSON file to a PDF file and exit
    Render(RenderArgs),
}

#[derive(Args)]
pub struct RenderArgs {
    /// Path to the resume JSON input file
    #[arg(short, long)]
    pub input: PathBuf,

    /// Path to write the output PDF to
    #[arg(short, long)]
    pub output: PathBuf,
}
