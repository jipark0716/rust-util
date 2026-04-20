mod command;

use clap::{Parser, Subcommand};
use crate::command::{decrypt, encrypt, app_setting_compare};

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
    AppSettingCompare(app_setting_compare::Args),
}


#[tokio::main]
async fn main() {
    let args = Command::parse();

    match args.command {
        Commands::Encrypt(c) => c.run(),
        Commands::Decrypt(c) => c.run(),
        Commands::AppSettingCompare(c) => c.run().await,
    }

}