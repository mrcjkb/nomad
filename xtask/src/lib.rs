#![allow(missing_docs)]

mod neovim;

const WORKSPACE_ROOT: &abs_path::AbsPath = {
    match abs_path::AbsPath::from_str(env!("CARGO_MANIFEST_DIR")) {
        Ok(manifest_dir) => manifest_dir.parent().expect("not root"),
        Err(_) => panic!("$CARGO_MANIFEST_DIR not absolute"),
    }
};

#[derive(clap::Parser)]
#[command(about)]
struct Args {
    #[command(subcommand)]
    editor: Editor,
}

#[derive(clap::Subcommand)]
enum Editor {
    #[command(subcommand)]
    Neovim(neovim::Command),
}

/// The entrypoint of the `xtask` binary.
pub fn run() -> anyhow::Result<()> {
    let args = <Args as clap::Parser>::parse();

    match args.editor {
        Editor::Neovim(command) => neovim::run(command),
    }
}
