#![allow(missing_docs)]

mod r#build;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(about)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Build the Nomad plugin.
    #[command(visible_alias = "b")]
    Build {
        /// Build the plugin in release mode.
        #[clap(long, short)]
        release: bool,

        /// Build the plugin for the latest nightly version of Neovim.
        #[clap(long)]
        nightly: bool,
    },
}

/// The entrypoint of the `xtask` binary.
pub fn run() -> anyhow::Result<()> {
    let args = Args::parse();

    match args.command {
        Command::Build { release, nightly } => build::build(release, nightly),
    }
}
