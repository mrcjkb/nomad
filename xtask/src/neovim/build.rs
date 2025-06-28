use core::{iter, str};
use std::{env, fs, process};

use abs_path::{AbsPath, AbsPathBuf, NodeNameBuf, node};
use anyhow::{Context, anyhow};
use cargo_metadata::TargetKind;

use crate::WORKSPACE_ROOT;
use crate::neovim::CARGO_TOML_META;

#[derive(Debug, Clone, clap::Args)]
pub(crate) struct BuildArgs {
    /// Build the plugin in release mode.
    #[clap(long, short, default_value_t = false)]
    release: bool,

    /// Build the plugin for the latest nightly version of Neovim.
    #[clap(long, default_value_t = false)]
    nightly: bool,

    /// The absolute path to the directory under which to place the build
    /// artifacts.
    #[clap(long, default_value_t = WORKSPACE_ROOT.to_owned())]
    out_dir: AbsPathBuf,
}

pub(crate) fn build(args: BuildArgs) -> anyhow::Result<()> {
    fs::create_dir_all(&args.out_dir)?;

    let artifact_dir = args.out_dir.clone().join(node!("lua"));

    // Setting the artifact directory is still unstable.
    let artifact_dir_args = ["-Zunstable-options", "--artifact-dir"]
        .into_iter()
        .chain(iter::once(artifact_dir.as_str()));

    let package_meta = &CARGO_TOML_META;

    // Specify which package to build.
    let package_args = ["--package", &package_meta.name].into_iter();

    let feature_args =
        args.nightly.then_some("--features=neovim-nightly").into_iter();

    let profile_args = args.release.then_some("--release").into_iter();

    let output = process::Command::new("cargo")
        .arg("build")
        .args(artifact_dir_args)
        .args(package_args)
        .args(feature_args)
        .args(profile_args)
        .stdout(process::Stdio::inherit())
        .stderr(process::Stdio::inherit())
        .output()?;

    if !output.status.success() {
        return Err(anyhow!(
            "cargo build failed with exit code {:?}: {}",
            output.status.code(),
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    fix_library_name(&artifact_dir)?;

    Ok(())
}

#[allow(clippy::unwrap_used)]
fn fix_library_name(artifact_dir: &AbsPath) -> anyhow::Result<()> {
    let package_meta = &CARGO_TOML_META;

    let mut cdylib_targets = package_meta.targets.iter().filter(|target| {
        target.kind.iter().any(|kind| kind == &TargetKind::CDyLib)
    });

    let cdylib_target = cdylib_targets.next().ok_or_else(|| {
        anyhow!(
            "Could not find a cdylib target in manifest of package {:?}",
            package_meta.name
        )
    })?;

    if cdylib_targets.next().is_some() {
        return Err(anyhow!(
            "Found multiple cdylib targets in manifest of package {:?}",
            package_meta.name
        ));
    }

    let source = format!(
        "{prefix}{lib_name}{suffix}",
        prefix = env::consts::DLL_PREFIX,
        lib_name = &cdylib_target.name,
        suffix = env::consts::DLL_SUFFIX
    )
    .parse::<NodeNameBuf>()
    .unwrap();

    let dest = format!(
        "{lib_name}{suffix}",
        lib_name = &cdylib_target.name,
        suffix = if cfg!(target_os = "windows") { ".dll" } else { ".so" }
    )
    .parse::<NodeNameBuf>()
    .unwrap();

    force_rename(&artifact_dir.join(&source), &artifact_dir.join(&dest))
        .context("Failed to rename the library")
}

fn force_rename(src: &AbsPath, dst: &AbsPath) -> anyhow::Result<()> {
    if fs::metadata(dst).is_ok() {
        fs::remove_file(dst)?;
    }
    fs::rename(src, dst)?;
    Ok(())
}
