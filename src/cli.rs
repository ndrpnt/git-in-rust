use crate::data;
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use std::{env, io};

#[derive(Debug, Parser)]
#[clap(name = "rustig")]
#[clap(about = "A git implemented in RUST", long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[clap()]
    Init {},
    #[clap(arg_required_else_help = true)]
    HashObject {
        #[clap(required = true, parse(from_os_str))]
        path: PathBuf,
    },
}

pub fn parse() {
    let args = Cli::parse();

    match args.command {
        Commands::Init {} => init().expect("Unable to init .rustig repository"),
        Commands::HashObject { path } => hash_object(path).expect("Unable to hash file"),
    }
}

fn init() -> io::Result<()> {
    data::init()?;
    let current_directory = env::current_dir()?;
    println!(
        "Initialized empty rustig repository in {}/{}",
        current_directory.display(),
        data::GIT_DIR
    );
    Ok(())
}

fn hash_object(path: PathBuf) -> io::Result<()> {
    let hash = data::hash_object(path)?;
    println!("generated hash: {}", hash);
    Ok(())
}
