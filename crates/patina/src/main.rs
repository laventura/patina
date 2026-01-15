//! Patina - A fast, lightweight Markdown editor
//!
//! TUI (Terminal User Interface) version

use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

mod app;
mod config;
mod input;
mod ui;

use app::App;

/// Patina - A fast, lightweight Markdown editor
#[derive(Parser, Debug)]
#[command(name = "patina")]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Files to open
    #[arg(value_name = "FILE")]
    files: Vec<PathBuf>,

    /// Open a folder as workspace
    #[arg(short, long, value_name = "DIR")]
    workspace: Option<PathBuf>,

    /// Use a specific config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    /// Set the color theme
    #[arg(short, long)]
    theme: Option<String>,

    /// Start in Zen mode
    #[arg(long)]
    zen: bool,

    /// Enable debug logging
    #[arg(long)]
    debug: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Initialize logging
    if cli.debug {
        env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();
    } else {
        env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("warn")).init();
    }

    log::info!("Starting Patina v{}", env!("CARGO_PKG_VERSION"));

    // Load configuration
    let config = config::Config::load(cli.config.as_deref())?;

    // Create and run the app
    let mut app = App::new(config)?;

    // Apply CLI options
    if let Some(theme) = cli.theme {
        app.set_theme(&theme);
    }
    if cli.zen {
        app.toggle_zen_mode();
    }

    // Open files or workspace
    if let Some(workspace_path) = cli.workspace {
        app.open_workspace(workspace_path)?;
    } else if cli.files.is_empty() {
        // Open empty document
        app.new_document();
    } else {
        for file in cli.files {
            app.open_file(file)?;
        }
    }

    // Run the main loop
    app.run()
}
