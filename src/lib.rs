//! cargo sandbox: perform Cargo builds inside of a sandboxed environment

#![doc(
    html_logo_url = "https://avatars3.githubusercontent.com/u/44121472",
    html_root_url = "https://docs.rs/cargo-sandbox/0.0.0"
)]
#![forbid(unsafe_code)]
#![warn(missing_docs, rust_2018_idioms, unused_qualifications)]

#[macro_use]
pub mod status;

pub mod error;
pub mod sandbox;
pub mod workspace;

pub use self::{
    error::{Error, ErrorKind},
    sandbox::SandboxCmd,
};
use gumdrop::Options;
use std::ffi::OsStr;

/// Run `cargo sandbox`
pub fn start(args: &[String]) -> Result<(), Error> {
    let (sandbox_args, cargo_args) = match args.iter().position(|arg| arg == "--") {
        Some(pos) => (&args[..pos], &args[pos..]),
        None => {
            if args.get(0).map(AsRef::as_ref) == Some("sandbox") {
                (&args[0..], &args[1..])
            } else {
                (&[] as &[String], args)
            }
        }
    };

    Command::parse_args_default(sandbox_args)
        .map_err(|e| ErrorKind::Argument.source(e))?
        .run(cargo_args)
}

/// Command-line option parser
#[derive(Debug, Options)]
pub enum Command {
    /// The `cargo sandbox` subcommand
    #[options(help = "perform a Cargo command inside of a sandbox")]
    Sandbox(SandboxCmd),
}

impl Command {
    /// Run the command
    pub fn run<S: AsRef<OsStr>>(&self, cargo_args: &[S]) -> Result<(), Error> {
        match self {
            Command::Sandbox(sandbox) => sandbox.run(cargo_args),
        }
    }
}
