//! Build workspaces

pub use rustwide::cmd::{SandboxBuilder, SandboxImage};

use crate::error::{Error, ErrorKind};
use rustwide::{Crate, Toolchain, WorkspaceBuilder};
use std::{ffi::OsStr, path::Path};

/// User-Agent to use when creating workspace
pub const USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

/// Build workspace.
///
/// This is a wrapper for [`rustwide::Workspace`]
pub struct Workspace {
    /// Inner Rustwide workspace
    inner: rustwide::Workspace,

    /// Toolchain to use when performing build
    toolchain: Toolchain,
}

impl Workspace {
    /// Initialize a build workspace
    pub fn new(
        path: impl AsRef<Path>,
        toolchain: Toolchain,
        image: SandboxImage,
    ) -> Result<Workspace, Error> {
        WorkspaceBuilder::new(path.as_ref(), USER_AGENT)
            .sandbox_image(image)
            .init()
            .map(|inner| Workspace { inner, toolchain })
            .map_err(|e| ErrorKind::Workspace.source(e))
    }

    /// Run Cargo within the workspace with the given argument
    pub fn run<S: AsRef<OsStr>>(
        &mut self,
        name: &str,
        path: impl AsRef<Path>,
        args: &[S],
    ) -> Result<(), Error> {
        let krate = Crate::local(path.as_ref());
        let builder = SandboxBuilder::new().enable_networking(false);
        let mut build_dir = self.inner.build_dir(name);

        build_dir
            .build(&self.toolchain, &krate, builder)
            .run(|build| build.cargo().args(args).run())
            .map_err(|e| ErrorKind::Build.source(e))
    }
}
