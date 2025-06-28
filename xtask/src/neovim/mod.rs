mod build;

#[derive(clap::Subcommand)]
pub(crate) enum Command {
    /// Build the Neovim plugin.
    #[command(visible_alias = "b")]
    Build(build::BuildArgs),
}

pub(crate) fn run(command: Command) -> anyhow::Result<()> {
    match command {
        Command::Build(args) => build::build(args),
    }
}
