use core::panic;

use clap::{Parser, Subcommand};
use eww_launch::{spawn_one_bar, spawn_two_bars};
use hyprland::Result;
use hyprland_window_title::listen_window_title;
use hyprland_workspaces::{listen_mon, listen_single_mon};

mod eww_launch;
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
    /// Launch eww bar(s)
    LaunchEww {
        /// Two comma-separated positions of the monitors. If left out, a bar for a single monitor is opened.
        #[clap(short, long, value_delimiter(','))]
        positions: Option<Vec<i32>>,
    },
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
        Commands::LaunchEww { positions: None } => spawn_one_bar(),
        Commands::LaunchEww {
            positions: Some(positions),
        } => match *positions.as_slice() {
            [left, right] => spawn_two_bars(left, right),
            _ => panic!("invalid argument format"),
        },

        Commands::WindowTitle => listen_window_title(),

        Commands::Workspaces { monitor: None } => listen_single_mon(),
        Commands::Workspaces {
            monitor: Some(monitor),
        } => listen_mon(&monitor),
    }
}
