//! The `cargo sandbox` subcommand

use crate::{
    error::{Error, ErrorKind},
    workspace::Workspace,
};
use gumdrop::Options;
use rustwide::{cmd::SandboxImage, Toolchain};
use std::{
    ffi::OsStr,
    path::PathBuf,
    process::{exit, Command},
};

/// The `cargo sandbox` subcommand
#[derive(Default, Debug, Options)]
pub struct SandboxCmd {
    /// Get help information
    #[options(short = "h", long = "help", help = "output help information and exit")]
    help: bool,

    /// Get version information
    #[options(no_short, long = "version", help = "output version and exit")]
    version: bool,
}

impl SandboxCmd {
    /// Run the `cargo sandbox` subcommand
    pub fn run<S: AsRef<OsStr>>(&self, cargo_args: &[S]) -> Result<(), Error> {
        if self.help {
            println!("{}", Self::usage());
            exit(0);
        }

        if self.version {
            println!("cargo-sandbox {}", env!("CARGO_PKG_VERSION"));
            exit(0);
        }

        // Initialize env-logger
        env_logger::builder()
            .filter_level("info".parse().unwrap())
            .format_timestamp(None)
            .init();

        // TODO(tarcieri): customize workspace directory
        let workspace_dir = self.workspace_dir()?;

        // TODO(tarcieri): customize build dir
        let build_dir_name = self.build_dir_name()?;

        // TODO(tarcieri): customize toolchain
        let toolchain = Toolchain::Dist {
            name: "stable".into(),
        };

        // TODO(tarcieri): customize sandbox image
        let image_name = "rustops/crates-build-env";
        status!("Fetching", "`{}` docker image", image_name);

        let image =
            SandboxImage::remote(image_name).map_err(|e| ErrorKind::DockerImage.source(e))?;

        status!(
            "Creating",
            "`{}` workspace ({})",
            workspace_dir.display(),
            toolchain
        );

        let mut workspace = Workspace::new(&workspace_dir, toolchain, image)?;

        status!(
            "Running",
            "cargo inside workspace: `{}`",
            workspace_dir.display(),
        );

        // TODO(tarcieri): determine project root instead of always using `.`
        workspace.run(&build_dir_name, ".", cargo_args)
    }

    /// Get the workspace directory
    fn workspace_dir(&self) -> Result<PathBuf, Error> {
        home::cargo_home()
            .map(|path| path.join("sandbox"))
            .map_err(|e| ErrorKind::Path.source(e))
    }

    /// Get the build directory for the current Cargo project
    fn build_dir_name(&self) -> Result<String, Error> {
        let pkgid_bytes = Command::new("cargo")
            .arg("pkgid")
            .output()
            .map_err(|e| ErrorKind::Cargo.source(e))?
            .stdout;

        let pkgid = String::from_utf8(pkgid_bytes).map_err(|e| ErrorKind::Cargo.source(e))?;

        let pkg_name_with_comment = pkgid
            .split("/")
            .last()
            .ok_or_else(|| Error::from(ErrorKind::Cargo))?;

        let pkg_name = pkg_name_with_comment
            .split("#")
            .next()
            .ok_or_else(|| Error::from(ErrorKind::Cargo))?;

        Ok(pkg_name.to_owned())
    }
}
