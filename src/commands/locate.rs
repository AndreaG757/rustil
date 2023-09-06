use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Instant;

use clap::Parser;
use colored::Colorize;
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
    pub directory: PathBuf,

    /// The file to find in the directory
    #[arg(short, long)]
    pub file: String,
}

// LOCATE-WORD
pub fn execute_locate_word(args: &LocateWordArgs) {
    walk_dir_word(&args.directory, &args.word, &args.extensions)
}

fn walk_dir_word(directory: &PathBuf, word: &str, file_types: &Option<String>) {
    if let Ok(entries) = fs::read_dir(directory) {
        entries
            .par_bridge()
            .filter_map(Result::ok)
            .for_each(|entry| {
                let path = entry.path();

                if let Ok(metadata) = path.metadata() {
                    if metadata.is_dir() {
                        walk_dir_word(&entry.path(), word, &file_types);
                    } else {
                        let file_name = entry.file_name();
                        if let Some(types) = file_types {
                            if types.split(",")
                                .filter(|ext| !ext.is_empty() && !ext.split_whitespace().next().is_none())
                                .any(|ext| file_name.to_string_lossy().ends_with(ext.trim())) {
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
                println!("Found in file: {}", dir.display().to_string().cyan());
                println!("Line: {} -  {}", line_number.to_string().bright_yellow(), item.replace(word, &word.yellow().bold()
                    .to_string()).trim().bright_black());
                println!();
            }
        }
    }
}

// LOCATE-FILE
pub fn execute_locate_file(args: &LocateFileArgs) {
    let start = Instant::now();
    let found_count = AtomicUsize::new(0);
    walk_dir_file(&args.directory, &args.file, &found_count);
    let final_count = found_count.load(Ordering::Relaxed);
    println!();

    if final_count > 0 {
        println!("Found {} files", final_count.to_string().green());
    } else {
        println!("{}", "Files not found".red());
    }

    println!();
    println!("Execution time: {:?}", start.elapsed())
}

fn walk_dir_file(directory: &PathBuf, file: &str, count: &AtomicUsize) {
    if let Ok(entries) = fs::read_dir(Path::new(directory)) {
        entries.par_bridge().filter_map(Result::ok).for_each(|entry| {
            let path = entry.path();
            if let Ok(metadata) = path.metadata() {
                if metadata.is_dir() {
                    walk_dir_file(&entry.path(), file, count);
                } else if path.file_name().unwrap_or_default() == file {
                    println!("{} {}", "Found in path:", path.display().to_string().bold().underline());
                    count.fetch_add(1, Ordering::Relaxed);
                }
            }
        })
    }
}
