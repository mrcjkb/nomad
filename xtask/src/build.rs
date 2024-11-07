use core::{fmt, str};

use anyhow::{anyhow, Context};
use fs::os_fs::OsFs;
use fs::{AbsPath, AbsPathBuf, FsNodeName};
use futures_executor::block_on;
use root_finder::markers;
use xshell::cmd;

pub(crate) fn build(_release: bool) -> anyhow::Result<()> {
    let sh = xshell::Shell::new()?;
    let project_root = find_project_root(&sh)?;
    let package_name = parse_package_name(&project_root)?;
    let nvim_version = detect_nvim_version(&sh)?;
    build_plugin(&project_root, &package_name, nvim_version, &sh)?;
    fix_library_name(&project_root, &package_name, &sh)?;
    Ok(())
}

fn find_project_root(sh: &xshell::Shell) -> anyhow::Result<AbsPathBuf> {
    let current_dir = sh.current_dir();
    let current_dir = <&AbsPath>::try_from(&*current_dir)?;
    let root_finder = root_finder::Finder::new(OsFs);
    block_on(root_finder.find_root(current_dir, markers::Git))?
        .ok_or_else(|| anyhow!("Could not find the project root"))
}

fn parse_package_name(project_root: &AbsPath) -> anyhow::Result<String> {
    let cargo_dot_toml = {
        let mut root = project_root.to_owned();
        #[allow(clippy::unwrap_used)]
        root.push(<&FsNodeName>::try_from("Cargo.toml").unwrap());
        root
    };
    let metadata = cargo_metadata::MetadataCommand::new()
        .manifest_path(cargo_dot_toml.clone())
        .exec()?;
    metadata.root_package().map(|p| p.name.to_owned()).ok_or_else(|| {
        anyhow!(
            "Could not find the root package for manifest at \
             {cargo_dot_toml:?}"
        )
    })
}

fn detect_nvim_version(sh: &xshell::Shell) -> anyhow::Result<NeovimVersion> {
    let version = "--version";
    let stdout = cmd!(sh, "nvim {version}").read()?;
    stdout
        .lines()
        .next()
        .ok_or_else(|| anyhow!("Couldn't get Neovim version"))?
        .split_once("NVIM v")
        .map(|(_, rest)| rest.parse::<NeovimVersion>())
        .transpose()?
        .ok_or_else(|| anyhow!("Failed to parse Neovim version"))
}

fn build_plugin(
    project_root: &AbsPathBuf,
    package_name: &str,
    nvim_version: NeovimVersion,
    sh: &xshell::Shell,
) -> anyhow::Result<()> {
    println!("project_root: {:?}", project_root);
    println!("package_name: {:?}", package_name);
    println!("nvim_version: {:?}", nvim_version);
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
#[derive(Debug)]
enum NeovimVersion {
    /// The latest stable version.
    ZeroDotTen,

    /// The latest nightly version.
    Nightly,
}

struct SemanticVersion {
    major: u8,
    minor: u8,
    patch: u8,
}

impl str::FromStr for NeovimVersion {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nightly_suffix = "-dev";
        let is_nightly = s.ends_with(nightly_suffix);
        let version = s
            [..s.len() - (is_nightly as usize) * nightly_suffix.len()]
            .parse::<SemanticVersion>()
            .context("Failed to parse Neovim version")?;
        if version.major == 0 && version.minor == 10 {
            Ok(Self::ZeroDotTen)
        } else if version.major == 0 && version.minor == 11 && is_nightly {
            Ok(Self::Nightly)
        } else {
            Err(anyhow!(
                "Unsupported Neovim version: {version}{}",
                if is_nightly { nightly_suffix } else { "" }
            ))
        }
    }
}

impl fmt::Display for SemanticVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl str::FromStr for SemanticVersion {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split('.');
        let major =
            parts.next().ok_or_else(|| anyhow!("major version is missing"))?;
        let minor =
            parts.next().ok_or_else(|| anyhow!("minor version is missing"))?;
        let patch =
            parts.next().ok_or_else(|| anyhow!("patch version is missing"))?;
        if parts.next().is_some() {
            return Err(anyhow!("too many version parts"));
        }
        Ok(Self {
            major: major.parse()?,
            minor: minor.parse()?,
            patch: patch.parse()?,
        })
    }
}
