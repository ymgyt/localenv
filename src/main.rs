#![allow(clippy::module_inception)]

mod cli;
mod config;
mod error;
mod operation;
mod prelude;
mod system;

// Parse command line args, then dispatch process.
async fn run() {
    let cmd = cli::initialize_from_args();

    init_logger(cmd.verbose);

    match cmd.subcommand {
        cli::SubCommand::Apply(opt) => cli::apply::run(opt).await,
    }
}

// Initialize global subscriber.
fn init_logger(verbose: u8) {
    tracing_subscriber::FmtSubscriber::builder()
        .without_time()
        .with_target(false)
        .with_env_filter(match verbose {
            0 => "localenv=info",
            1 => "localenv=debug",
            2 => "localenv=trace",
            _ => "trace",
        })
        .init()
}

fn main() {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async { run().await })
}
