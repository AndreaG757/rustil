use args::{RustilArgs, ActionType};
use clap::Parser;

mod commands;
mod args;

fn main() {
    let args = RustilArgs::parse();

    match args.action {
        ActionType::LocateWord(locate_word_args) => {
            commands::locate::execute_locate_word(&locate_word_args);
        },
        ActionType::LocateFile(locate_file_args) => {
            commands::locate::execute_locate_file(&locate_file_args);
        }
    }
}