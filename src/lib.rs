mod cli;

use std::{io, env};

use clap::Parser;
use walkdir::WalkDir;

#[derive(thiserror::Error, Debug)]
pub enum NucleError {
    #[error("IO-Error occured: {0}")]
    IO(#[from] io::Error),
}

pub fn run() -> Result<(), NucleError> {
    let args = cli::Cli::parse();
    let dir_walker = get_dir_walker()?;
    Ok(())
}

fn get_dir_walker() -> Result<WalkDir, io::Error> {
    let current_dir = env::current_dir()?;
    Ok(WalkDir::new(current_dir))
}
