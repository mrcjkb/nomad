use std::io;

use fs::AbsPathBuf;

pub(crate) fn build(_release: bool) -> anyhow::Result<()> {
    let sh = xshell::Shell::new()?;
    let project_root = find_project_root(&sh)?;
    let package_name = parse_package_name(&project_root, &sh)?;
    let nvim_version = detect_nvim_version(&sh)?;
    build_plugin(&project_root, &package_name, nvim_version, &sh)?;
    fix_library_name(&project_root, &package_name, &sh)?;
    Ok(())
}

fn find_project_root(sh: &xshell::Shell) -> io::Result<AbsPathBuf> {
    todo!();
}

fn parse_package_name(
    project_root: &AbsPathBuf,
    sh: &xshell::Shell,
) -> anyhow::Result<String> {
    todo!();
}

fn detect_nvim_version(sh: &xshell::Shell) -> anyhow::Result<NeovimVersion> {
    todo!();
}

fn build_plugin(
    project_root: &AbsPathBuf,
    package_name: &str,
    nvim_version: NeovimVersion,
    sh: &xshell::Shell,
) -> anyhow::Result<()> {
    todo!();
}

fn fix_library_name(
    project_root: &AbsPathBuf,
    package_name: &str,
    sh: &xshell::Shell,
) -> anyhow::Result<()> {
    todo!();
}

/// The possible Neovim versions the Nomad plugin can be built for.
enum NeovimVersion {
    /// The latest stable version.
    ZeroDotTen,

    /// The latest nightly version.
    Nightly,
}
