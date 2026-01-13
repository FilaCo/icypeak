use clap::Parser;
use icypeak::cli::Cli;
use tracing::{info, trace};

#[tokio::main]
async fn main() {
    // getting guard is necessary, because it needs to be alive during the whole `main` execution
    let _guard = icypeak::tracing::init();
    info!("app starting");
    let cli = Cli::parse();
    trace!("command line args parsed");
    cli.handle().await;
}
