use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct LocateWordArgs {
    /// The directory that can contain the word in a file
    #[arg(short, long)]   
    pub directory: String,

    /// The word to find in the directory
    #[arg(short, long)]
    pub word: String
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct LocateFileArgs {
    /// The directory that can contain the word in a file
    #[arg(short, long)]   
    pub directory: String,

    /// The file to find in the directory
    #[arg(short, long)]
    pub file: String
}

pub fn execute_locate_word(args: &LocateWordArgs) {
    println!("execute_locate_word {}, {}", args.directory, args.word)
}

pub fn execute_locate_file(args: &LocateFileArgs) {
    println!("execute_locate_file {}, {}", args.directory, args.file)
}
