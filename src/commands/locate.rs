use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use clap::Parser;
use rayon::prelude::*;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct LocateWordArgs {
    /// The directory that can contain the word in a file
    #[arg(short, long)]   
    pub directory: PathBuf,

    /// The word to find in the directory
    #[arg(short, long)]
    pub word: String,

    /// The file type where to check
    #[arg(short, long)]
    pub extensions: Option<String>,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct LocateFileArgs {
    /// The directory that can contain the word in a file
    #[arg(short, long)]   
    pub directory: String,

    /// The file to find in the directory
    #[arg(short, long)]
    pub file: String,
}

pub fn execute_locate_word(args: &LocateWordArgs) {
    walk_dir(&args.directory, &args.word, &args.extensions)
}

fn walk_dir(directory: &PathBuf, word: &str, file_types: &Option<String>) {
    if let Ok(entries) = fs::read_dir(directory) {
        entries
            .par_bridge()
            .filter_map(Result::ok)
            .for_each(|entry| {
                let path = entry.path();
                if entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false) {
                    walk_dir(&entry.path(), word, &file_types);
                } else {
                    let file_name = entry.path().display().to_string();

                    if let Some(file_type) = file_types {
                        if file_type.contains(",") {
                            let type_list = file_type.split(",");
                            type_list
                                .map(|ty| ty.trim())
                                .filter(|ty| !ty.is_empty())
                                .for_each(|ty| {
                                    if file_name.contains(ty) {
                                        search_word(&path, word)
                                    }
                            })
                        } else {
                            if let Some(file_type) = file_types {
                                if file_name.contains(file_type) {
                                    search_word(&path, word)
                                }
                            }
                        }
                    } else {
                        if let Some(types) = file_types {
                            if file_name.contains(types) {
                                search_word(&path, word)
                            }
                        } else {
                            search_word(&path, word)
                        }
                    }
                }
        });
    }
}

fn search_word(dir: &PathBuf, word: &str) {
    let file = File::open(dir).unwrap();
    let reader = BufReader::with_capacity(8192, file);

    for (line_number, line) in reader.lines().enumerate() {
        if let Ok(item) = line {
            if item.contains(word) {
                println!();
                println!("Found in file: {}", dir.display());
                println!("line: {} {}", line_number, item);
                println!();
            }
        }
    }
}

pub fn execute_locate_file(args: &LocateFileArgs) {
    println!("execute_locate_file {}, {}", args.directory, args.file)
}
