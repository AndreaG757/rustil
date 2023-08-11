use clap::{
    Parser,
    Subcommand
};

use crate::commands::locate::{LocateWordArgs, LocateFileArgs};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct RustilArgs {
    #[clap(subcommand)]
    pub action: ActionType
}

#[derive(Debug, Subcommand)]
pub enum ActionType {
    /// Locate a word in files in the dir
    LocateWord(LocateWordArgs),

    /// Locate a file in the dir
    LocateFile(LocateFileArgs),
}
