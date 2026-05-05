mod cli;
mod compiler;
mod config;
mod error;
mod resume;
mod server;
mod str_to_content;
mod typst_writer;

use cli::{Cli, Command, RenderArgs};
use compiler::Compiler;
use resume::ResumeData;

use std::{
    fs,
    io::{self, IsTerminal},
};

use anyhow::Result;
use clap::Parser;
use tokio::{net::TcpListener, signal};
use tracing::info;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    let cli = Cli::parse();

    match cli.command {
        Command::Serve => run_serve().await,
        Command::Render(args) => run_render(&args),
    }
}

async fn run_serve() -> Result<()> {
    let cfg = config::Config::load()?;

    init_tracing(&cfg.rust_log);

    let compiler = Compiler::new();
    let router = server::router(compiler);

    let addr = format!("{}:{}", cfg.host, cfg.port);
    let listener = TcpListener::bind(&addr).await?;

    info!("Listening on {addr}");
    info!("OpenAPI docs at http://{addr}/docs");

    axum::serve(listener, router)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

fn run_render(args: &RenderArgs) -> Result<()> {
    init_tracing("info");

    let json_bytes = fs::read(&args.input)?;
    let data: ResumeData = serde_json::from_slice(&json_bytes)?;

    let compiler = Compiler::new();
    let pdf = compiler.compile(&data)?;

    fs::write(&args.output, &pdf)?;

    info!(
        input = %args.input.display(),
        output = %args.output.display(),
        bytes = pdf.len(),
        "PDF written"
    );

    Ok(())
}

fn init_tracing(rust_log: &str) {
    let filter = EnvFilter::try_new(rust_log).unwrap_or_else(|_| EnvFilter::new("info"));

    if io::stderr().is_terminal() {
        tracing_subscriber::fmt().with_env_filter(filter).init();
    } else {
        tracing_subscriber::fmt()
            .json()
            .with_env_filter(filter)
            .init();
    }
}

async fn shutdown_signal() {
    signal::ctrl_c().await.expect("Failed to listen for Ctrl-C");
    info!("Shutting down");
}
