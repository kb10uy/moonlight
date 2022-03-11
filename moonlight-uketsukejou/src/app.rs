//! Contains application arguments and state definitions.

use clap::Parser;

/// Integrated suite for Moonlight.
#[derive(Debug, Clone, Parser)]
#[clap(author, version, about)]
pub struct Arguments {
    /// Subcommand to execute.
    #[clap(subcommand)]
    pub subcommand: Subcommand,
}

/// Subcommand definition.
#[derive(Debug, Clone, Parser)]
pub enum Subcommand {
    /// Tests the transmission path throughout the Internet.
    TestPath(TestPathArguments),
}

/// Arguments set for `TestPath`.
#[derive(Debug, Clone, Parser)]
pub struct TestPathArguments {
    /// Filename of transmission path.
    #[clap(short, long)]
    pub script: String,

    /// Values for test.
    pub values: Vec<f64>,
}
