mod cli;

use clap::Parser;

#[derive(thiserror::Error, Debug, Clone, PartialEq, Eq, Hash)]
pub enum NucleError {}

pub fn run() -> Result<(), NucleError> {
    let args = cli::Cli::parse();
    Ok(())
}
