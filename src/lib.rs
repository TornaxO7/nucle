mod cli;
mod tui;

use std::{env, io};

use clap::Parser;
use tui::Tui;
use async_walkdir::WalkDir;

#[derive(thiserror::Error, Debug)]
pub enum NucleError {
    #[error("An IO-Error occured: {0}")]
    IO(#[from] io::Error),
}

pub async fn run() -> Result<(), NucleError> {
    let args = cli::Cli::parse();
    let walkdir = get_walkdir()?;
    let mut tui = Tui::new()?;

    tui.run(walkdir).await;
    Ok(())
}

fn get_walkdir() -> Result<WalkDir, io::Error> {
    let current_dir = env::current_dir()?;
    Ok(WalkDir::new(current_dir))
}
