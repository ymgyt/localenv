mod subcommands;
pub use subcommands::*;

mod helper;

use structopt::{clap::AppSettings, StructOpt};

const ABOUT_LOCALENV: &str ="\
    localenv is a command line tool to provision your local development environment with declarative manner.
";

#[derive(StructOpt, Debug)]
#[structopt(
    name = "localenv",
    about = ABOUT_LOCALENV,
    version (env ! ("CARGO_PKG_VERSION")),
    setting(AppSettings::ArgRequiredElseHelp),
    global_settings(& [
        AppSettings::ColoredHelp,
        AppSettings::ColorAuto,
        AppSettings::VersionlessSubcommands,
        AppSettings::DisableHelpSubcommand,
        AppSettings::DeriveDisplayOrder,
    ])
)]
pub struct LocalEnv {
    #[structopt(
        short = "v",
        long = "verbose",
        global = true,
        help = "set logging level.(v=DEBUG, vv=TRACE)",
        parse(from_occurrences)
    )]
    pub verbose: u8,

    #[structopt(subcommand)]
    pub subcommand: SubCommand,
}

pub fn initialize_from_args() -> LocalEnv {
    LocalEnv::from_args()
}

#[derive(StructOpt, Debug, Clone)]
pub enum SubCommand {
    Apply(subcommands::Apply),
    Plan(subcommands::Plan),
}

