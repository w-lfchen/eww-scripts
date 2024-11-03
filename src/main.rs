use clap::{Parser, Subcommand};
use hyprland::Result;
use hyprland_window_title::listen_window_title;
use hyprland_workspaces::{listen_mon, listen_single_mon};

mod hyprland_window_title;
mod hyprland_workspaces;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Get the current window title, intended for eww `deflisten`
    WindowTitle,
    /// Get monitor-specific workspace status, intended for eww `deflisten`
    Workspaces {
        /// Monitor to get the status for. If left out, the existence of only one monitor is assumed.
        #[clap(short, long)]
        monitor: Option<String>,
    },
}

fn main() -> Result<()> {
    match Cli::parse().command {
        Commands::WindowTitle => listen_window_title(),

        Commands::Workspaces { monitor: None } => listen_single_mon(),
        Commands::Workspaces {
            monitor: Some(monitor),
        } => listen_mon(&monitor),
    }
}
