use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "critter", author = "Hemenguelbindi", version="0.0.1")]
pub struct CritterCommand {
    #[command(subcommand)]
    pub commands: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    StartCritter {
        name: String,
    },
    Status,
    StopCritter,
}
