use clap::Parser;
use bookrlib::Cli;
use bookrlib::phonebook::Phonebook;
use bookrlib::Commands;



fn main(){
    let cli = Cli::parse();

    let mut book = Phonebook::new();

    match &cli.command {
        Commands::Add(contact) => {
            book.add_contact(&contact.name, &contact.number);
        }
        Commands:: Edit(val) => {
            println!("Edit functionality not implemented.")
        }
        Commands:: Lookup(search) => {
            if let Some(contact) = book.get_contact(&search.name) {
                println!("Name: {} Phone number: {}", contact.name, contact.phone);
            } else {
                println!("Name not found in phonebook");
            }

        }
    }
}