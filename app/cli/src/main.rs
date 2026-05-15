mod command;

use clap::{Parser, Subcommand};
use crate::command::{decrypt, encrypt};

#[derive(Parser)]
#[command(name = "cli")]
struct Command {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Encrypt(encrypt::Args),
    Decrypt(decrypt::Args),
}


#[tokio::main]
async fn main() {
    let args = Command::parse();

    let result = match args.command {
        Commands::Encrypt(c) => c.run(),
        Commands::Decrypt(c) => c.run(),
    };

    if let Err(e) = result {
        println!("error: {}", e);
    }
}