mod cli;
mod tui;

use std::{env, io::{self, Write}, process::Command};

use clap::Parser;
use tui::Tui;

#[derive(thiserror::Error, Debug)]
pub enum NucleError {
    #[error("An IO-Error occured: {0}")]
    IO(#[from] io::Error),
}

pub fn run() -> Result<(), NucleError> {
    let args = cli::Cli::parse();
    let mut tui = Tui::new()?;

    tui.run();
    Ok(())
}
