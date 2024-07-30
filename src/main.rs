use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
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

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::LaunchEww { positions: _ } => {
            todo!()
        }
        Commands::WindowTitle => {
            todo!()
        }
        Commands::Workspaces { monitor: _ } => {
            todo!()
        }
    }
}
