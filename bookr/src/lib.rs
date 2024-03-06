pub mod phonebook;

use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(name = "bookr")]
#[command(version = "1.0")]
#[command(about = "A simple phonebook", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands
}

#[derive(Subcommand)]
pub enum Commands {
    /// Look up a contact's phone number
    Lookup (LookupArgs),
    /// Edit a contact's name or number
    Edit (EditArgs),
    /// Add a new contact. If you want to update an existing contact use edit.
    Add (AddArgs)
}

#[derive(Args)]
pub struct LookupArgs {
    pub name: String
}

#[derive(Args)]
pub struct AddArgs {
    pub name: String,
    pub number: String
}

#[derive(Args)]
pub struct EditArgs {
    pub name: String,
    pub number: String
}