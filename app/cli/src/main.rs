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


fn main() {
    let args = Command::parse();

    match args.command {
        Commands::Encrypt(c) => c.run(),
        Commands::Decrypt(c) => c.run(),
    }

}