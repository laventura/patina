//! Patina GUI - Desktop version using egui
//!
//! This is a placeholder for v0.5 - GUI support coming soon!

use anyhow::Result;
use clap::Parser;

/// Patina GUI - A fast, lightweight Markdown editor (Desktop version)
#[derive(Parser, Debug)]
#[command(name = "patina-gui")]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Files to open
    #[arg(value_name = "FILE")]
    files: Vec<std::path::PathBuf>,
}

fn main() -> Result<()> {
    let _cli = Cli::parse();

    println!("╔═══════════════════════════════════════════════════════════╗");
    println!("║                                                           ║");
    println!("║   🎨 Patina GUI                                           ║");
    println!("║                                                           ║");
    println!("║   Desktop version coming in v0.5!                         ║");
    println!("║                                                           ║");
    println!("║   For now, use the TUI version:                           ║");
    println!("║   $ patina <file.md>                                      ║");
    println!("║                                                           ║");
    println!("╚═══════════════════════════════════════════════════════════╝");

    Ok(())
}
