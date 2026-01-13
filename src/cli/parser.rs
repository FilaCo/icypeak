use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about)]
#[command(propagate_version = true)]
pub struct Cli {
    /// Detach icypeak app from the current terminal
    #[arg(short, long)]
    pub detach: bool,
}
