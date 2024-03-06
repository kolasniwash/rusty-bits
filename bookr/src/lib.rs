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
    #[command(arg_required_else_help = true)]
    Lookup(LookupArgs),
    /// Add a new contact. If you want to update an existing contact use edit.
    #[command(arg_required_else_help = true)]
    Add(AddArgs),
    /// Edit a contact's name or number
    Edit(EditArgs),

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
// #[command(args_conflicts_with_subcommands = true)]
pub struct  EditArgs {
    #[command(subcommand)]
    pub command: EditCommands
}

#[derive(Subcommand)]
pub enum EditCommands {
    name(EditNameArgs),
    number(EditNumberArgs)
}

#[derive(Args)]
pub struct EditNameArgs {
    pub name: String,
    pub new_name: String
}

#[derive(Args)]
pub struct EditNumberArgs {
    pub name: String,
    pub number: String
}