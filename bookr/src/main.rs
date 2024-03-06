use std::fs::{File, write};
use std::io::{BufReader, BufWriter};
use std::path::Path;
use clap::Parser;

use std::error::Error;

use bookrlib::Cli;
use bookrlib::phonebook::Phonebook;
use bookrlib::Commands;


const PHONEBOOK_PATH: &str = "phonebook.json";
fn get_phonebook() -> Result<Phonebook, Box<dyn Error>>{

    if Path::new(PHONEBOOK_PATH).exists(){
        println!("Loading phonebook.json");
    }
    let mut file:File = File::open(PHONEBOOK_PATH)?;
    let reader = BufReader::new(file);
    let book: Phonebook = serde_json::from_reader(reader)?;
    Ok(book)

    // println!("Creating new phonebook.json");
    // let book = Phonebook::new();
    // Ok(book)
}

fn save_phonebook(book: &mut Phonebook) -> std::io::Result<()> {
    let file = File::create(PHONEBOOK_PATH)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer(writer, book)?;
    Ok(())
}

fn main() -> std::io::Result<()>{

    let cli = Cli::parse();
    let mut book:Phonebook = get_phonebook().unwrap_or(Phonebook::new());

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

    // save updated phonebook
    save_phonebook(&mut book)?;
    Ok(())
}