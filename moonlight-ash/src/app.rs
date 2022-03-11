use clap::Parser;

/// Moonlight Encoder Utility
#[derive(Debug, Parser)]
#[clap(author, version)]
pub struct Arguments {
    /// Subcommand to execute.
    #[clap(subcommand)]
    pub subcommand: Subcommand,
}

/// Subcommand arguments.
#[derive(Debug, Parser)]
pub enum Subcommand {
    /// (For debugging) Logs tracked device activities.
    DebugTrackDevices,
}
