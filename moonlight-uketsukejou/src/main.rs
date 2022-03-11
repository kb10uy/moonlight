mod app;
mod luautil;
mod subcommand;

use crate::{
    app::{Arguments, Subcommand},
    subcommand::test_path,
};

use anyhow::Result;
use clap::Parser;
use flexi_logger::{colored_detailed_format, Logger};

#[async_std::main]
async fn main() -> Result<()> {
    Logger::try_with_env()?
        .format_for_stderr(colored_detailed_format)
        .start()?;

    let args = Arguments::parse();
    match args.subcommand {
        Subcommand::TestPath(args) => test_path::run(args).await?,
    }

    Ok(())
}
